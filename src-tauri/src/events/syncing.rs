use std::{fs, path::Path};

use libp2p::{gossipsub::IdentTopic, PeerId, Swarm};
use mongodb::{
    bson::{doc, to_document, Document},
    Collection, Database,
};
use serde::{Deserialize, Serialize};
use sp_core::ed25519::Public;
use tauri::Emitter;

use crate::{
    generator::{
        block::{block::Block, message::BlockMessage},
        leader::Leader,
        relay::Relay,
        swarm::CentichainBehaviour,
        transaction::Transaction,
        validator::Validator,
    },
    tools::{
        bsons::Bson,
        downloader::Downloader,
        trun_sync::{Sync, Turn},
        zipp::Zip,
    },
};

use super::gossip_messages::handler::GossipMessages;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VSync {
    relay: PeerId,
    peerid: PeerId,
    msg: String,
    wallet: Public,
}

impl VSync {
    pub fn new(relay: &PeerId, peerid: &PeerId, msg: String, wallet: &Public) -> Self {
        Self {
            relay: *relay,
            peerid: *peerid,
            msg,
            wallet: *wallet,
        }
    }

    //handle gossip messages that are VSync model
    pub async fn handle<'a>(
        &self,
        db: &'a Database,
        window: &tauri::Window,
    ) -> Result<(), &'a str> {
        match Validator::new(db, self.peerid, self.relay, self.wallet).await {
            Ok(validator) => {
                let collection: Collection<Document> = db.collection("validators");
                let filter = to_document(&validator).unwrap();
                let query = collection.find_one(filter).await;

                //check validators and if it doesn't include new node peer id then insert new node as validator
                if let Ok(Some(_doc)) = query {
                    Ok(())
                } else {
                    match collection
                        .insert_one(to_document(&validator).unwrap())
                        .await
                    {
                        Ok(_) => Ok(window
                            .emit(
                                "status",
                                &format!("New synced validator added: {}", self.peerid),
                            )
                            .unwrap()),
                        Err(_) => {
                            Err("Error while inserting new validator-(relay/tools/syncer 63)")
                        }
                    }
                }
            }
            Err(e) => Err(e),
        }
    }

    //propagate sync message to network in client topic
    pub fn propagate<'a>(
        &self,
        swarm: &mut Swarm<CentichainBehaviour>,
        window: &tauri::Window,
    ) -> Result<(), &'a str> {
        let gossipmessage = GossipMessages::SyncMessage(self.clone());
        let message = serde_json::to_string(&gossipmessage).unwrap();

        match swarm
            .behaviour_mut()
            .gossipsub
            .publish(IdentTopic::new("validator"), message)
        {
            Ok(_) => Ok(window
                .emit("status", "Sync message sent to the relay")
                .unwrap()),
            Err(_e) => Err("Propagation Sync Message Error!"),
        }
    }

    //download and extract the blockchain and then insert it to database
    async fn get_blockchain<'a>(window: &tauri::Window, db: &'a Database) -> Result<(), &'a str> {
        //get connected relay ip and make blockchain download link from it
        let relay_ip = Relay::ip_adress(db).await;
        match relay_ip {
            Ok(splited_addr) => {
                //start downloading blockchain from connected relay
                let url = format!("http://{}:33369/blockchain/Centichain.zip", splited_addr);
                match Downloader::download(&url, "Centichain.zip", window).await {
                    Ok(_) => match Zip::extract("./etc/dump/Centichain", window) {
                        Ok(_) => Ok(()),
                        Err(e) => Err(e),
                    },
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    //after getting blockchain and unzip it to bson files syncing start inserting thos into database
    pub async fn insert_bsons<'a>(
        window: &tauri::Window,
        db: &'a Database,
        mempool: &mut Vec<Transaction>,
    ) -> Result<(), &'a str> {
        let mut error = None;

        // Get blockchain and unzip it
        match Self::get_blockchain(window, db).await {
            Ok(_) => {
                //add bsons to database or mempool by a loop
                let path = "./etc/dump/Centichain";
                for entry in fs::read_dir(path).unwrap() {
                    let item = entry.unwrap();
                    let file_name = item.file_name();
                    let collection_name =
                        Path::new(&file_name).file_stem().unwrap().to_str().unwrap();
                    let str_name = file_name.to_str().unwrap();
                    if str_name.contains("bson") {
                        if str_name == "transactions.bson" {
                            match Bson::add(None, None, Some(mempool), &str_name, window).await {
                                Ok(_) => {}
                                Err(e) => {
                                    error = Some(e);
                                    break;
                                }
                            }
                        } else {
                            match Bson::add(
                                Some(db),
                                Some(&collection_name),
                                None,
                                &str_name,
                                window,
                            )
                            .await
                            {
                                Ok(_) => {}
                                Err(e) => {
                                    error = Some(e);
                                    break;
                                }
                            }
                        }
                    }
                }

                //if error was some return it and if not continues syncing
                if error.is_some() {
                    Err(error.unwrap())
                } else {
                    Ok(window
                        .emit("status", "Blockchain inserted successfully")
                        .unwrap())
                }
            }
            Err(e) => Err(e),
        }
    }

    pub async fn checking_turn<'a>(
        db: &'a Database,
        mempool: &mut Vec<Transaction>,
        turn: &mut Turn,
        leader: &mut Leader,
        wallet: &Public,
        peerid: &PeerId,
        private: &String,
        last_block: &mut Vec<Block>,
        relay: &mut Relay,
        window: &tauri::Window,
        swarm: &mut Swarm<CentichainBehaviour>,
        sync_state: &mut Sync,
    ) -> Result<(), &'a str> {
        let collection: Collection<Document> = db.collection("validators");
        let validators_count = collection.count_documents(doc! {}).await;
        match validators_count {
            Ok(count) => {
                if count == 0 {
                    turn.on(window);
                    leader.update(None, window);
                    window.emit("turn", turn.shift.to_string()).unwrap();

                    if mempool.len() > 1 {
                        //push 51 transaction to block's transactions from mempool
                        let mut transactions = Vec::new();
                        while mempool.len() > 0 {
                            if transactions.len() < 51 {
                                transactions.push(mempool.remove(0));
                            } else {
                                break;
                            }
                        }

                        //make new block mssage (include new block and next leader)
                        match BlockMessage::new(
                            db,
                            transactions,
                            wallet,
                            peerid,
                            private,
                            last_block,
                            relay,
                            turn,
                            leader,
                            window,
                            sync_state,
                        )
                        .await
                        {
                            Ok(block_message) => {
                                window.emit("mempool", mempool.clone()).unwrap(); //show mempool in front
                                block_message.post(db, swarm, relay).await
                            }
                            Err(e) => Err(e),
                        }
                    } else {
                        window.emit("turn", turn.shift.to_string()).unwrap();
                        Ok(())
                    }
                } else {
                    turn.off(count as u16, window);
                    window.emit("turn", turn.shift.to_string()).unwrap();
                    Ok(())
                }
            }
            Err(_) => Err("Problem during get count of validators-(events/syncing 188)"),
        }
    }
}
