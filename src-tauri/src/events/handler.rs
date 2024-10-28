use std::{thread, time::Duration};

use libp2p::{
    futures::StreamExt,
    request_response::{Event, Message},
    swarm::SwarmEvent,
    PeerId, Swarm,
};
use mongodb::Database;
use sp_core::ed25519::Public;
use tauri::Emitter;

use crate::{
    generator::{
        block::{block::Block, message::BlockMessage},
        leader::Leader,
        relay::Relay,
        swarm::{CentichainBehaviour, CentichainBehaviourEvent},
    },
    tools::trun_sync::{Sync, Turn},
};

use super::{
    gossip_messages::handler::GossipMessages, handshaking::Handshake,
    outgoing_connection::OutgoingConnection, response::Responses, syncing::VSync,
};

pub async fn handle(
    swarm: &mut Swarm<CentichainBehaviour>,
    window: &tauri::Window,
    db: &Database,
    peerid: &PeerId,
    wallet: &Public,
    private: &String,
) {
    // Retrieve relay information from the database
    match Relay::find(&db).await {
        Ok(mut relay) => {
            // Initialize necessary components
            let mut turn = Turn::new();
            let mut mempool = Vec::new();
            let mut recieved_blocks: Vec<BlockMessage> = Vec::new();
            let mut last_block: Vec<Block> = Vec::new();
            let mut sync_state = Sync::new();
            let mut leader = Leader::new(None, window);

            // Main event handling loop
            'handler: loop {
                match swarm.select_next_some().await {
                    // Handle new connection establishment
                    SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                        // Emit connection information to the UI
                        window.emit("relay", format!("{}", peer_id)).unwrap();
                        window.emit("peerid", format!("{}", peerid)).unwrap();

                        // Update relay peer ID in the database
                        match Relay::update(&mut relay, &db, Some(peer_id), None).await {
                            Ok(_) => {}
                            Err(e) => window.emit("error", e).unwrap(),
                        }
                    }

                    // Handle outgoing connection errors
                    SwarmEvent::OutgoingConnectionError { peer_id, .. } => {
                        match OutgoingConnection::delete_post(db).await {
                            Ok(_) => window
                                .emit(
                                    "status",
                                    format!("Dialing failed with:{}", peer_id.unwrap()),
                                )
                                .unwrap(),
                            Err(e) => window.emit("error", e).unwrap(),
                        }
                        break 'handler;
                    }

                    // Handle connection closure
                    SwarmEvent::ConnectionClosed { .. } => {
                        break 'handler;
                    }

                    // Handle behavior events
                    SwarmEvent::Behaviour(events) => match events {
                        // Handle request-response events
                        CentichainBehaviourEvent::Reqres(reqres) => match reqres {
                            Event::Message { message, .. } => match message {
                                Message::Response { response, .. } => {
                                    let cloned_relay = relay.clone();
                                    match sync_state {
                                        Sync::NotSynced => {
                                            // Handle response for not synced state
                                            match Responses::handle(
                                                window,
                                                response,
                                                db,
                                                wallet,
                                                peerid,
                                                &mut turn,
                                                private,
                                                &mut mempool,
                                                &mut recieved_blocks,
                                                &mut last_block,
                                                &mut relay,
                                                swarm,
                                                &mut sync_state,
                                                &mut leader,
                                            )
                                            .await
                                            {
                                                Ok(_) => {
                                                    // Check turn after successful response handling
                                                    match VSync::checking_turn(
                                                        db,
                                                        &mut mempool,
                                                        &mut turn,
                                                        &mut leader,
                                                        wallet,
                                                        peerid,
                                                        private,
                                                        &mut last_block,
                                                        &mut relay,
                                                        window,
                                                        swarm,
                                                        &mut sync_state,
                                                    )
                                                    .await
                                                    {
                                                        Ok(_) => {}
                                                        Err(e) => {
                                                            // Disconnect on error
                                                            swarm
                                                                .disconnect_peer_id(
                                                                    relay.peerid.unwrap(),
                                                                )
                                                                .unwrap();
                                                            window.emit("error", e).unwrap()
                                                        }
                                                    }

                                                    // Wait for a short period before updating UI
                                                    thread::sleep(Duration::from_secs(7));
                                                    // Update UI with current mempool and turn information
                                                    window
                                                        .emit("mempool", mempool.clone())
                                                        .unwrap();
                                                    window.emit("patience", turn.waiting).unwrap();
                                                }
                                                Err(e) => {
                                                    // Disconnect on error
                                                    swarm
                                                        .disconnect_peer_id(
                                                            cloned_relay.peerid.unwrap(),
                                                        )
                                                        .unwrap();
                                                    window.emit("error", e).unwrap();
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            },
                            _ => {}
                        },

                        // Handle gossip messages
                        CentichainBehaviourEvent::Gossipsub(gossipsub) => match gossipsub {
                            libp2p::gossipsub::Event::Subscribed { peer_id, topic } => {
                                // Add explicit peer for validator topic
                                if topic.to_string() == "validator".to_string() {
                                    swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                                    Handshake::start(relay.peerid.unwrap(), swarm, window);
                                }
                            }
                            libp2p::gossipsub::Event::Message {
                                message,
                                propagation_source,
                                ..
                            } => {
                                // Handle incoming gossip messages
                                match GossipMessages::handle(
                                    message.data,
                                    propagation_source,
                                    swarm,
                                    window,
                                    db,
                                    &mut mempool,
                                    &mut turn,
                                    wallet,
                                    peerid,
                                    private,
                                    &mut last_block,
                                    &mut relay.clone(),
                                    &mut leader,
                                    &mut sync_state,
                                    &mut recieved_blocks,
                                )
                                .await
                                {
                                    Ok(_) => {}
                                    Err(e) => {
                                        // Exit on error
                                        window.emit("error", e).unwrap();
                                        std::process::exit(0)
                                    }
                                }
                            }
                            _ => {}
                        },
                    },
                    _ => {}
                }
            }
        }
        Err(e) => window.emit("error", e).unwrap(),
    }
}
