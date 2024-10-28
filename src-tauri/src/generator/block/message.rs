use libp2p::{futures::StreamExt, PeerId, Swarm};
use mongodb::{
    bson::{doc, from_document, Document},
    Collection, Database,
};
use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};
use sp_core::ed25519::Public;
use tauri::Emitter;

use crate::{
    events::handshaking::Requests,
    generator::{
        leader::Leader,
        relay::Relay,
        swarm::{CentichainBehaviour, Req},
        transaction::Transaction,
        validator::Validator,
    },
    tools::{
        trun_sync::{Sync, Turn},
        wrongdoer::WrongDoer,
    },
};

use super::block::Block;

// Struct representing a block message in the network
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockMessage {
    pub block: Block,
    pub next_leader: PeerId,
}

impl BlockMessage {
    // Create a new gossip message for propagation to the network
    pub async fn new<'a>(
        db: &'a Database,
        transactions: Vec<Transaction>,
        wallet: &Public,
        peerid: &PeerId,
        private: &String,
        last_block: &mut Vec<Block>,
        relay: &mut Relay,
        turn: &mut Turn,
        leader: &mut Leader,
        window: &tauri::Window,
        sync_state: &mut Sync,
    ) -> Result<Self, &'a str> {
        match Block::new(
            &db,
            transactions,
            wallet,
            peerid,
            private,
            last_block,
            relay,
            window,
            turn,
            sync_state,
        )
        .await
        {
            Ok(block) => match Self::find_next_leader(db, *peerid, turn, leader, window).await {
                Ok(next_leader) => {
                    leader.peerid.get_or_insert(next_leader); // Set leader peer id
                    window.emit("block", "+").unwrap(); // Show new block in front-end
                    window.emit("genBlock", "New block is created").unwrap();
                    Ok(Self { block, next_leader }) // Return block message
                }
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    // Handle received block messages
    pub async fn handle<'a>(
        self,
        swarm: &mut Swarm<CentichainBehaviour>,
        peerid: &PeerId,
        db: &'a Database,
        recvied_blocks: &mut Vec<Self>,
        sync_state: &mut Sync,
        last_block: &mut Vec<Block>,
        mempool: &mut Vec<Transaction>,
        leader: &mut Leader,
        source: PeerId,
        relay: &mut Relay,
        window: &tauri::Window,
        turn: &mut Turn,
        wallet: &Public,
    ) -> Result<(), &'a str> {
        if self.block.header.signature.key != *wallet {
            // If leader is true then validate block
            if leader.peerid.is_none() || self.block.header.validator == leader.peerid.unwrap() {
                match sync_state {
                    // If validator is synced then validate block
                    Sync::Synced => match self
                        .block
                        .validation(last_block, db, mempool, wallet, window, turn, sync_state)
                        .await
                    {
                        Ok(_) => {
                            window.emit("mempool", mempool.clone()).unwrap(); // Show mempool in front-end
                            window.emit("block", "+").unwrap(); // Show new block in front-end

                            match self.block.insertion(db).await {
                            Ok(_) => {
                                // If next leader was peer id, turn on and update leader
                                if &self.next_leader == peerid {
                                    leader.update(None, window);
                                    turn.on(window);
                                    Ok(())
                                } else {
                                    Ok(leader.update(Some(self.next_leader), window)) // If block was valid and inserted to DB then change leader 
                                }
                            }
                            Err(_) => Err("Error while inserting new block to database-(generator/block/message 101)")
                        }
                        }
                        Err(e) => {
                            if source == relay.peerid.unwrap() {
                                window.emit("error", e).unwrap();
                                swarm.disconnect_peer_id(relay.peerid.unwrap()).unwrap(); // If message source was validator's relay then connection must be disconnected
                                Ok(())
                            } else {
                                window.emit("status", e).unwrap();
                                leader
                                    .start_voting(db, swarm, peerid, window, turn, sync_state)
                                    .await
                                // If message source was not validator's relay then voting of leader must be started
                            }
                        }
                    },
                    // If validator is not synced, received message is pushed to received blocks for syncing
                    Sync::NotSynced => Ok(recvied_blocks.push(self)),
                }
            } else {
                match WrongDoer::remove(db, self.block.header.validator, turn, sync_state, window)
                    .await
                {
                    Ok(peer) => Ok(window
                        .emit("status", format!("This wrongdoer removed: {}", peer))
                        .unwrap()),
                    Err(e) => Err(e),
                }
            }
        } else {
            Ok(())
        }
    }

    // Find next leader and return it
    pub async fn find_next_leader<'a>(
        db: &'a Database,
        peerid: PeerId,
        turn: &mut Turn,
        leader: &mut Leader,
        window: &tauri::Window,
    ) -> Result<PeerId, &'a str> {
        let mut in_turn_validators = Vec::new(); // Define it for get validators that their waiting is 0 and push to it
        let collection: Collection<Document> = db.collection("validators");

        // Get count of validators that their waiting is 0 and if there is not return current leader peer id
        match collection.count_documents(doc! {"waiting": 0}).await {
            Ok(count) => {
                if count > 0 {
                    match collection.find(doc! {"waiting": 0}).await {
                        Ok(mut cursor) => {
                            while let Some(Ok(doc)) = cursor.next().await {
                                let validator: Validator = from_document(doc).unwrap();
                                in_turn_validators.push(validator); // Push validator that its waiting is 0 to in turns
                            }

                            // Choose a validator at random
                            let rnd_validator = in_turn_validators
                                .iter()
                                .choose(&mut rand::thread_rng())
                                .unwrap();

                            let count_of_validators =
                                collection.count_documents(doc! {}).await.unwrap();
                            turn.off(count_of_validators as u16, window); // Turn off and set waiting of relay to count of validators

                            // Update leader
                            leader.update(Some(rnd_validator.peerid), window);

                            Ok(rnd_validator.peerid) // Return random validator
                        }
                        Err(_) => Err("Error while finding in turn validators"),
                    }
                } else {
                    // Turning on and leader updates
                    turn.on(window);
                    leader.update(None, window);

                    Ok(peerid) // Return our peer id as next leader
                }
            }
            Err(_) => Err("Database error while getting count-(generator/block/message 175)"),
        }
    }

    // Post the block message to the network
    pub async fn post<'a>(
        self,
        db: &'a Database,
        swarm: &mut Swarm<CentichainBehaviour>,
        relay: &Relay,
    ) -> Result<(), &'a str> {
        let request = Requests::BlockMessage(self.clone());
        let req = serde_json::to_string(&request).unwrap(); // Serialize block message

        let request = Req { req }; // Define new request

        swarm
            .behaviour_mut()
            .reqres
            .send_request(&relay.peerid.unwrap(), request); // Send new request to relay

        self.block.insertion(db).await // Insert block to database after sending it
    }
}
