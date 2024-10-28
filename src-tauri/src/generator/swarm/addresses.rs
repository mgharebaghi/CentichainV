use std::net::TcpStream;

use libp2p::{futures::StreamExt, Multiaddr, Swarm};
use mongodb::{
    bson::{doc, from_document, to_document, Document},
    Collection, Database,
};
use serde::Deserialize;

use rand::seq::SliceRandom;
use tauri::Emitter;

use crate::generator::relay::Relay;

use super::CentichainBehaviour;

#[derive(Debug, Deserialize)]
pub struct Addresses {
    status: String,
    data: Vec<Address>,
}

#[derive(Debug, Deserialize)]
struct Address {
    addr: String,
}

impl Addresses {
    //get 50 addresses in max from centichain server
    pub async fn get<'a>(db: &Database) -> Result<(), &'a str> {
        let response = reqwest::get("https://centichain.org/api/relays").await;
        match response {
            Ok(data) => {
                let collection: Collection<Document> = db.collection("relays");
                match serde_json::from_str::<Addresses>(&data.text().await.unwrap()) {
                    Ok(res) => {
                        if res.status == "success" && res.data.len() > 0 {
                            for relay in res.data {
                                let new_relay = Relay::new(None, String::new(), relay.addr);
                                let doc = to_document(&new_relay).unwrap();
                                collection.insert_one(doc).await.unwrap();
                            }
                            Ok(())
                        } else {
                            Err("there is no any relay in the network! please try later.")
                        }
                    }
                    Err(_e) => Err("Error while cast response to json - addresses(47)"),
                }
            }
            Err(_) => Err("Error from getting data - addresses(50)"),
        }
    }

    //contacting to a random address
    pub async fn contact<'a>(
        swarm: &mut Swarm<CentichainBehaviour>,
        db: &Database,
        window: &tauri::Window,
    ) -> Result<(), &'a str> {
        //check internet connection and if it connection is stable then start dial with relays as random
        window.emit("status", "Check your internet...").unwrap();
        let internet_connection = TcpStream::connect("8.8.8.8:53");

        if let Ok(_) = internet_connection {
            window.emit("status", "Your internet is connected").unwrap();
            //check count of relays and if there are any relays in the network then start dialing to a random relay
            window.emit("status", "Checking for relays...").unwrap();
            let collection: Collection<Document> = db.collection("relays");
            let count_docs = collection.count_documents(doc! {}).await;
            match count_docs {
                Ok(count) => {
                    if count > 0 {
                        Self::contacting(collection, swarm, db, window).await
                    } else {
                        match Self::get(&db).await {
                            Ok(_) => Self::contacting(collection, swarm, db, window).await,
                            Err(e) => Err(e),
                        }
                    }
                }
                Err(_) => Err("problem in counting of database documents-(addresses-82)"),
            }
        } else {
            Err("internet connection lost, please check your internet-(addressess-86)")
        }
    }

    //contacting method
    async fn contacting<'a>(
        collection: Collection<Document>,
        swarm: &mut Swarm<CentichainBehaviour>,
        db: &Database,
        window: &tauri::Window,
    ) -> Result<(), &'a str> {
        window
            .emit("status", "Relays found, Start dialing...")
            .unwrap();
        let mut relays: Vec<Relay> = Vec::new();
        let mut cursor = collection.find(doc! {}).await.unwrap();

        while let Some(doc) = cursor.next().await {
            let relay: Relay = from_document(doc.unwrap()).unwrap();
            relays.push(relay);
        }

        //choos a relay as random for dialing
        let random_relay = relays.choose(&mut rand::thread_rng()).unwrap();
        //delete from DB
        let deleted = collection
            .delete_one(doc! {"addr": random_relay.addr.to_string()})
            .await;
        //if deleted was ok then dialing will satrts after delete previous connected relay in realy collection
        if let Ok(_) = deleted {
            let relay_coll: Collection<Document> = db.collection("relay");
            match relay_coll.delete_many(doc! {}).await {
                Ok(_) => {
                    let doc = to_document(random_relay).unwrap();
                    match relay_coll.insert_one(doc).await {
                        Ok(_) => Ok(swarm
                            .dial(random_relay.addr.parse::<Multiaddr>().unwrap())
                            .unwrap()),
                        Err(_) => {
                            Err("error from insert of relay into mongodb-(swarm/addresses 125)")
                        }
                    }
                }
                Err(_) => Err("Deleting connected relays problem-(swarm/addresses 129)"),
            }
        } else {
            Err("random relay has problem for deleting-(swarm/dialing 132)")
        }
    }
}
