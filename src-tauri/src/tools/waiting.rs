use libp2p::{futures::StreamExt, PeerId};
use mongodb::{
    bson::{doc, from_document, to_document, Document},
    Collection, Database,
};

use crate::generator::validator::Validator;

use super::trun_sync::{Sync, Turn};

pub struct Waiting;

impl Waiting {
    pub async fn update<'a>(
        db: &'a Database,
        block_generator: &PeerId,
        turn: &mut Turn,
        window: &tauri::Window,
        sync_state: &mut Sync,
    ) -> Result<(), &'a str> {
        let collection: Collection<Document> = db.collection("validators");
        let query = collection.find(doc! {}).await;
        match query {
            Ok(mut cursor) => {
                let mut is_err = None;
                while let Some(Ok(doc)) = cursor.next().await {
                    let mut validator: Validator = from_document(doc.clone()).unwrap();
                    // Check if the validator is the block generator
                    if &validator.peerid == block_generator {
                        // Set waiting to the total number of validators
                        validator.waiting =
                            collection.count_documents(doc! {}).await.unwrap() as u64;
                        let replacement = to_document(&validator).unwrap();
                        match collection.replace_one(doc, replacement).await {
                            Ok(_) => {}
                            Err(_) => {
                                is_err.get_or_insert(
                                    "Error during the replacing of document-(tools/waiting 30)",
                                );
                                break;
                            }
                        }
                    } else {
                        // Decrease waiting by 1 if it's greater than 0
                        if validator.waiting > 0 {
                            validator.waiting -= 1;
                            let replacement = to_document(&validator).unwrap();
                            match collection.replace_one(doc, replacement).await {
                                Ok(_) => {}
                                Err(_) => {
                                    is_err.get_or_insert(
                                        "Error during the replacing of document-(tools/waiting 44)",
                                    );
                                    break;
                                }
                            }
                        }
                    }
                }
                if is_err.is_none() {
                    match sync_state {
                        Sync::Synced => {
                            // Update turn waiting if it's greater than 0
                            if turn.waiting > 0 {
                                turn.waiting_update(window);
                            }
                        }
                        _ => {}
                    }
                    Ok(())
                } else {
                    Err(is_err.unwrap())
                }
            }
            Err(_) => Err("Error during quering in-(tools/waiting 58)"),
        }
    }

    pub async fn new<'a>(db: &'a Database) -> Result<u64, &'a str> {
        let collection: Collection<Document> = db.collection("validators");
        let coun_query = collection.count_documents(doc! {}).await;
        match coun_query {
            Ok(count) => {
                if count > 0 {
                    // Set initial waiting to twice the number of validators
                    Ok(count)
                } else {
                    Ok(0)
                }
            }
            Err(_) => Err("Error while get count of validators-(relay/tools/waiting 73)"),
        }
    }
}
