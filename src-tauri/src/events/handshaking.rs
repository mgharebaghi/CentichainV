use libp2p::{PeerId, Swarm};
use mongodb::{
    bson::{doc, from_document, Document},
    options::FindOneOptions,
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
        swarm::{CentichainBehaviour, Req, Res},
        transaction::Transaction,
    },
    tools::trun_sync::{Sync, Turn},
};

use super::syncing::VSync;

#[derive(Debug, Serialize, Deserialize)]
pub enum Requests {
    Handshake(String),
    BlockMessage(BlockMessage),
    Transaction(Transaction),
}

//handshake response structure
#[derive(Debug, Deserialize)]
pub struct Handshake {
    wallet: String,
    first_node: FirstChecker,
}

//firstnode enum for handshake response that if it was yes it means the validator is first validator in the network
#[derive(Debug, Deserialize)]
enum FirstChecker {
    Yes,
    No,
}

impl Handshake {
    //send a handshake request to relay for know that there is any validator in the network or not
    //get the resoponse in event handler of libp2p
    pub fn start(peerid: PeerId, swarm: &mut Swarm<CentichainBehaviour>, window: &tauri::Window) {
        window.emit("status", "Handshaking...").unwrap();
        let handshake = Requests::Handshake("handshake".to_string());
        let str_handhsake = serde_json::to_string(&handshake).unwrap();
        let req = Req { req: str_handhsake };

        swarm.behaviour_mut().reqres.send_request(&peerid, req);
    }

    //get handshake resposne and deserialize it to handshake struct then check if the node is first node in the network generates Genesis Block
    //if its not first node start syncing
    pub async fn response<'a>(
        window: &tauri::Window,
        response: Res,
        db: &'a Database,
        wallet: &Public,
        peerid: &PeerId,
        private: &String,
        turn: &mut Turn,
        mempool: &mut Vec<Transaction>,
        recieved_blocks: &mut Vec<BlockMessage>,
        last_block: &mut Vec<Block>,
        relay: &mut Relay,
        leader: &mut Leader,
        swarm: &mut Swarm<CentichainBehaviour>,
        sync_state: &mut Sync,
    ) -> Result<(), &'a str> {
        let res: Self = serde_json::from_str(&response.res).unwrap();
        //update coneected relay wallet that is in the response from the relay in the database
        match Relay::update(relay, db, None, Some(res.wallet)).await {
            Ok(_) => {
                match res.first_node {
                    //if validatore is first in the network,it must makes the Genesis Block and propagates it
                    FirstChecker::Yes => {
                        match BlockMessage::new(
                            db,
                            Vec::new(),
                            wallet,
                            peerid,
                            private,
                            &mut Vec::new(),
                            relay,
                            turn,
                            leader,
                            window,
                            sync_state,
                        )
                        .await
                        {
                            Ok(block_message) => block_message.post(db, swarm, relay).await,
                            Err(e) => Err(e),
                        }
                    }

                    //if validator was not first in the network then it hav to syncs with the network
                    FirstChecker::No => {
                        //insert bsons that downloaded from relay then check recieved new blocks during the insert bson for syncing with the network
                        match VSync::insert_bsons(window, db, mempool).await {
                            Ok(_) => {
                                let mut is_err = None;
                                //finding last block after inserted bsons
                                let collection: Collection<Document> = db.collection("Blocks");
                                let options = FindOneOptions::builder()
                                    .sort(doc! {"header.number": -1})
                                    .build();
                                let last_block_doc = collection
                                    .find_one(doc! {})
                                    .with_options(options)
                                    .await
                                    .unwrap()
                                    .unwrap();
                                //pushing last block
                                let deserialized_block_doc: Block =
                                    from_document(last_block_doc).unwrap();
                                last_block.clear();
                                last_block.push(deserialized_block_doc);
                                //validating each block in recieved block during the inserting bsons
                                for i in 0..recieved_blocks.len() {
                                    match Block::validation(
                                        &recieved_blocks[i].block,
                                        last_block,
                                        &db,
                                        mempool,
                                        wallet,
                                        window,
                                        turn,
                                        sync_state,
                                    )
                                    .await
                                    {
                                        Ok(block) => {
                                            leader.update(
                                                Some(recieved_blocks[i].next_leader),
                                                window,
                                            );

                                            //insert block to the database
                                            match block.insertion(db).await {
                                                Ok(_) => {}
                                                Err(e) => {
                                                    is_err.get_or_insert(e);
                                                    break;
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            is_err.get_or_insert(e);
                                            break;
                                        }
                                    }
                                }

                                //if there is no any errors in recieved_blocks then syncing complete message send to front
                                //and handler will propagate syncing message to the network after get the OK in the response match
                                if is_err.is_none() {
                                    recieved_blocks.clear();
                                    Ok(window.emit("status", "Syncing Completed").unwrap())
                                } else {
                                    Err(is_err.unwrap())
                                }
                            }
                            Err(e) => Err(e),
                        }
                    }
                }
            }
            Err(e) => Err(e),
        }
    }
}
