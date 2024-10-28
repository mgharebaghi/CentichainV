use mongodb::{
    bson::{doc, from_document, Document},
    Collection, Database,
};
use reqwest::Client;

use crate::generator::relay::Relay;

pub struct OutgoingConnection;

impl OutgoingConnection {
    pub async fn delete_post<'a>(db: &'a Database) -> Result<(), &'a str> {
        let collection: Collection<Document> = db.collection("relay");
        let filter = doc! {};
        let query = collection.find_one(filter).await;
        if let Ok(Some(doc)) = query {
            let relay: Relay = from_document(doc).unwrap();
            let addr = relay.addr;

            let client = Client::new();
            let url = format!("https://centichain.org/api/relays?addr={}", addr);
            match client.delete(url).send().await {
                Ok(res) => {
                    if res.text().await.unwrap() == "success".to_string() {
                        Ok(())
                    } else {
                        Err("response has error from server during deleting address!")
                    }
                }
                Err(_e) => Err("delete post poblem!"),
            }
        } else {
            Ok(())
        }
    }
}
