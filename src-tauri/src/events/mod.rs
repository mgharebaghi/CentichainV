use std::str::FromStr;

use crate::generator::swarm::{CentichainBehaviour, Features};
pub mod db;
mod handler;
use db::DatabseConnection;
use handler::handle;
use sp_core::ed25519;
use tauri::Emitter;
pub mod gossip_messages;
pub mod handshaking;
mod outgoing_connection;
pub mod response;
pub mod syncing;

#[tauri::command]
pub async fn start(private: String, public: String, window: tauri::Window) {
    let wallet = ed25519::Public::from_str(&public).unwrap();
    //check database connection and if there was a problem pass an error to front
    match DatabseConnection::connect().await {
        //this loop is for repeat dialing with other relays if connection with relay closed in handler
        Ok(db) => loop {
            
            // Drop the existing database
            match db.drop().await {
                Ok(_) => {}
                Err(e) => {
                    println!("drop database error: {}", e);
                }
            
            }
            //config swarm with swarm mod
            let (mut swarm, peerid) = CentichainBehaviour::new().await;
            match CentichainBehaviour::dial(&mut swarm, &db, &window).await {
                //handle events of network if dialing was ok
                Ok(_) => handle(&mut swarm, &window, &db, &peerid, &wallet, &private).await,
                Err(e) => {
                    window.emit("error", e.to_string()).unwrap();
                    break;
                }
            }
        },
        Err(e) => window.emit("error", e).unwrap(),
    }
}
