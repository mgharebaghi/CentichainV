use libp2p::futures::StreamExt;
use mongodb::{
    bson::{doc, from_document, Document},
    options::FindOptions,
    Collection,
};

use crate::{events::db::DatabseConnection, generator::block::block::Block};

#[tauri::command]
pub async fn latest_blocks(page: u64) -> Vec<Block> {
    match DatabseConnection::connect().await {
        Ok(db) => {
            let mut latests = Vec::new();
            let collection: Collection<Document> = db.collection("Blocks");
            let sort = doc! {"header.number": -1}; // Sort by block number in descending order
            let skip = (page * 4) - 4; // Calculate the number of documents to skip
            let options = FindOptions::builder()
                .sort(sort)
                .skip(skip)
                .limit(4) // Limit the result to 10 documents
                .build();
            let cursor = collection.find(doc! {}).with_options(options).await;
            if let Ok(mut finder) = cursor {
                while let Some(Ok(doc)) = finder.next().await {
                    let block: Block = from_document(doc).unwrap(); // Convert document to Block
                    latests.push(block);
                }
                latests
            } else {
                latests
            }
        }
        Err(_e) => Vec::new(), // Return an empty vector if the connection fails
    }
}
