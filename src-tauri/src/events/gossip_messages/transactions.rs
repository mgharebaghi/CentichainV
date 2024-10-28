use chrono::Utc;
use libp2p::{PeerId, Swarm};
use mongodb::{
    bson::{doc, from_document, Document},
    Collection, Database,
};
use rust_decimal::Decimal;
use sp_core::ed25519::Public;
use tauri::Emitter;

use crate::{
    generator::{
        block::{block::Block, message::BlockMessage},
        leader::{Leader, LeaderTime},
        relay::Relay,
        swarm::CentichainBehaviour,
        transaction::Transaction,
    },
    tools::{
        trun_sync::{Sync, Turn},
        utxo::Person,
        wrongdoer::WrongDoer,
    },
};

pub struct Transactions;

impl Transactions {
    pub async fn handle<'a>(
        window: &tauri::Window,
        swarm: &mut Swarm<CentichainBehaviour>,
        transaction: Transaction,
        mempool: &mut Vec<Transaction>,
        turn: &mut Turn,
        db: &'a Database,
        wallet: &Public,
        peerid: &PeerId,
        private: &String,
        last_block: &mut Vec<Block>,
        relay: &mut Relay,
        source: PeerId,
        leader: &mut Leader,
        sync_state: &mut Sync,
    ) -> Result<(), &'a str> {
        // Validate the transaction
        match transaction.validate(db).await {
            Ok(_) => {
                // Check if the transaction is from the current wallet
                if transaction.signature[0].key == *wallet {
                    let sum_centies = Self::sum_centies(db, wallet).await;
                    match sum_centies {
                        Ok(sum) => {
                            // Update the UI with the new balance
                            window.emit("sum_centies", sum).unwrap();
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }

                // Add the validated transaction to the mempool
                mempool.push(transaction);
                // Update the UI with the new mempool state
                window.emit("mempool", mempool.clone()).unwrap();

                // Check if we need to create a new block or change leader
                if mempool.len() > 1 && !leader.in_check {
                    match leader.timer {
                        LeaderTime::On => {
                            let now = Utc::now();

                            // Check if the leader's time has expired
                            if now > leader.time.unwrap() {
                                // Initiate leader change process
                                match Leader::start_voting(
                                    leader, db, swarm, peerid, window, turn, sync_state,
                                )
                                .await
                                {
                                    Ok(_) => Ok(()),
                                    Err(e) => Err(e),
                                }
                            } else {
                                Ok(())
                            }
                        }

                        LeaderTime::Off => {
                            if turn.shift {
                                // Create a new block with up to 51 transactions from the mempool
                                let mut transactions = Vec::new();
                                while transactions.len() < 51 && !mempool.is_empty() {
                                    transactions.push(mempool.remove(0));
                                }

                                // Create and post a new block message
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
                                        // Update the UI with the new mempool state
                                        window.emit("mempool", mempool.clone()).unwrap();
                                        block_message.post(db, swarm, relay).await
                                    }
                                    Err(e) => Err(e),
                                }
                            } else {
                                // Start the leader's timer
                                leader.timer_start();
                                Ok(())
                            }
                        }
                    }
                } else {
                    Ok(())
                }
            }
            Err(e) => {
                // Handle invalid transaction
                if source == relay.peerid.unwrap() {
                    // Disconnect from the source if it's our relay
                    window.emit("error", e).unwrap();
                    swarm.disconnect_peer_id(source).unwrap();
                    Ok(())
                } else {
                    // Remove the wrongdoer from validators
                    match WrongDoer::remove(db, source, turn, sync_state, window).await {
                        Ok(wrongdoer) => Ok(window
                            .emit(
                                "status",
                                format!("This wrongdoer removed: {}", wrongdoer.to_string()),
                            )
                            .unwrap()),
                        Err(e) => Err(e),
                    }
                }
            }
        }
    }

    // Calculate the sum of unspent transaction outputs (UTXOs) for a given wallet
    pub async fn sum_centies<'a>(db: &'a Database, wallet: &Public) -> Result<String, &'a str> {
        let collection: Collection<Document> = db.collection("UTXOs");
        let filter = doc! {"wallet": wallet.to_string()};
        let query = collection.find_one(filter).await;
        if let Ok(Some(doc)) = query {
            let person: Person = from_document(doc).unwrap();
            // Sum up all unspent outputs
            let sum: Decimal = person.utxos.iter().map(|utxo| utxo.unspent).sum();
            Ok(sum.to_string())
        } else {
            Err("Problem in querying UTXOs-(events/gossip_messages/transactions.rs 141)")
        }
    }
}
