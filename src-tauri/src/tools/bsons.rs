use std::{fs::File, io::BufReader};

use mongodb::{
    bson::{from_document, Document},
    Collection, Database,
};
use tauri::Emitter;

use crate::generator::transaction::Transaction;

pub struct Bson;

impl Bson {
    pub async fn add<'a>(
        db: Option<&Database>,
        collection_name: Option<&str>,
        mut mempool: Option<&mut Vec<Transaction>>,
        bson: &str,
        window: &tauri::Window,
    ) -> Result<(), &'a str> {
        // Open the BSON file located at the specified address
        let bson_addr = format!("./etc/dump/Centichain/{}", bson);
        let open_file = File::open(bson_addr);
        match open_file {
            Ok(file) => {
                let mut reader = BufReader::new(file);

                // If a database is provided, remove all documents from the specified collection
                // and insert documents from the BSON file into the collection
                if let Some(db) = db {
                    let collection_name = collection_name.unwrap();
                    
                    // Drop the collection if it exists
                    db.collection::<Document>(collection_name).drop().await.ok();
                    
                    // Create a new collection
                    let collection: Collection<Document> = db.collection(collection_name);
                    
                    // Read documents from the BSON file and insert them into the collection
                    while let Ok(doc) = Document::from_reader(&mut reader) {
                        collection.insert_one(doc).await.unwrap();
                    }
                    
                    // Emit a status event to the window
                    window
                        .emit("status", format!("{} Synced", collection_name))
                        .unwrap();
                    Ok(())
                } else {
                    // If no database is provided, treat the BSON file as a transactions file
                    // and insert the transactions into the mempool
                    if let Some(ref mut orig_mempool) = mempool {
                        // Read documents from the BSON file and convert them to transactions
                        while let Ok(doc) = Document::from_reader(&mut reader) {
                            let transaction: Transaction = from_document(doc).unwrap();
                            orig_mempool.push(transaction);
                        }
                        Ok(window.emit("mempool", orig_mempool.clone()).unwrap())
                    } else {
                        Err("You dont set mempool!")
                    }
                }
            }
            Err(_e) => Err("Your file address is incorrect!-(tools/bsons 57)"),
        }
    }
}
