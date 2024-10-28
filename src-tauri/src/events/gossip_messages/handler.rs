use libp2p::{PeerId, Swarm};
use mongodb::Database;
use serde::{Deserialize, Serialize};
use sp_core::ed25519::Public;
use tauri::Emitter;

use crate::{
    events::syncing::VSync,
    generator::{
        block::{block::Block, message::BlockMessage},
        leader::Leader,
        relay::Relay,
        swarm::CentichainBehaviour,
        transaction::Transaction,
    },
    tools::{
        trun_sync::{Sync, Turn},
        wrongdoer::WrongDoer,
    },
};

use super::transactions::Transactions;

// Enum representing different types of gossip messages in the network
#[derive(Debug, Serialize, Deserialize)]
pub enum GossipMessages {
    BlockMessage(BlockMessage),
    Transaction(Transaction),
    SyncMessage(VSync),
    LeaderVote(PeerId),
    Outnode(PeerId),
}

impl GossipMessages {
    // Main handler for processing incoming gossip messages
    pub async fn handle<'a>(
        message: Vec<u8>,
        source: PeerId,
        swarm: &mut Swarm<CentichainBehaviour>,
        window: &tauri::Window,
        db: &'a Database,
        mempool: &mut Vec<Transaction>,
        turn: &mut Turn,
        wallet: &Public,
        peerid: &PeerId,
        private: &String,
        last_block: &mut Vec<Block>,
        relay: &mut Relay,
        leader: &mut Leader,
        sync_state: &mut Sync,
        recieved_blocks: &mut Vec<BlockMessage>,
    ) -> Result<(), &'a str> {
        // Convert message bytes to string
        if let Ok(str_message) = String::from_utf8(message) {
            // Deserialize the message string into a GossipMessages enum
            if let Ok(gossip_messag) = serde_json::from_str::<Self>(&str_message) {
                match gossip_messag {
                    // Handle incoming block messages
                    GossipMessages::BlockMessage(block_message) => {
                        block_message
                            .handle(
                                swarm,
                                peerid,
                                db,
                                recieved_blocks,
                                sync_state,
                                last_block,
                                mempool,
                                leader,
                                source,
                                relay,
                                window,
                                turn,
                                wallet,
                            )
                            .await
                    }

                    // Handle incoming transaction messages
                    GossipMessages::Transaction(transaction) => {
                        Transactions::handle(
                            window,
                            swarm,
                            transaction,
                            mempool,
                            turn,
                            db,
                            wallet,
                            peerid,
                            private,
                            last_block,
                            relay,
                            source,
                            leader,
                            sync_state,
                        )
                        .await
                    }

                    // Handle incoming sync messages
                    GossipMessages::SyncMessage(vsync) => {
                        match sync_state {
                            Sync::Synced => {
                                // Add new validator to validators document if it was a correct message
                                vsync.handle(db, window).await
                            }
                            _ => Ok(()),
                        }
                    }

                    // Handle incoming leader vote messages
                    GossipMessages::LeaderVote(vote) => {
                        match sync_state {
                            Sync::Synced => {
                                if leader.in_check {
                                    // Process the vote if leader is being checked
                                    leader.check_votes(db, vote, peerid, turn, window).await
                                } else {
                                    Ok(())
                                }
                            }
                            _ => Ok(()),
                        }
                    }

                    // Handle incoming outnode messages
                    GossipMessages::Outnode(outnode) => {
                        if leader.peerid.is_some() && outnode == leader.peerid.unwrap() {
                            // Start voting for a new leader if the current leader is the outnode
                            leader
                                .start_voting(db, swarm, &peerid, window, turn, sync_state)
                                .await
                        } else {
                            // Remove the outnode from the validators list
                            match WrongDoer::remove(db, outnode, turn, sync_state, window).await {
                                Ok(outnode) => Ok(window
                                    .emit("status", format!("{} is outnode", outnode.to_string()))
                                    .unwrap()),
                                Err(e) => Err(e),
                            }
                        }
                    }
                }
            } else {
                // Return Ok if the message couldn't be deserialized
                Ok(())
            }
        } else {
            // Return Ok if the message couldn't be converted to a string
            Ok(())
        }
    }
}
