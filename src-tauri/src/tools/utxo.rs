use mongodb::{
    bson::{doc, from_document, to_document, Document},
    Collection, Database,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use sp_core::ed25519::Public;
use tauri::Emitter;

use crate::{events::gossip_messages::transactions::Transactions, generator::transaction::Unspent};

// Represents a person with their wallet and UTXOs
#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub wallet: Public,
    pub utxos: Vec<UTXO>,
}

impl Person {
    fn new(wallet: Public, utxos: Vec<UTXO>) -> Self {
        Self { wallet, utxos }
    }
}

// Define a UTXO model that includes the transaction hash, the unspent value,
// the output hash of its transaction, and the block number of the transaction
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UTXO {
    pub block: u64,
    pub trx_hash: String,
    pub output_hash: String,
    pub unspent_hash: String,
    #[serde_as(as = "DisplayFromStr")]
    pub unspent: Decimal,
}

impl UTXO {
    // Check if a UTXO exists and remove it if it does
    pub async fn check<'a>(&self, db: &Database, wallet: &Public) -> Result<(), &'a str> {
        let collection: Collection<Document> = db.collection("UTXOs");
        let filter = doc! {"wallet": wallet.to_string()};
        let query = collection.find_one(filter).await;
        match query {
            Ok(opt) => match opt {
                Some(doc) => {
                    // Convert document to person structure
                    let mut person: Person = from_document(doc).unwrap();
                    // Find index of UTXO in the person's UTXOs
                    let index = person
                        .utxos
                        .iter()
                        .position(|u| u.unspent_hash == self.unspent_hash);

                    if let Some(i) = index {
                        let rm_utxo = person.utxos.remove(i);
                        // Update command to remove UTXO from database
                        let update =
                            doc! {"$pull": {"utxos": {"unspent_hash": rm_utxo.unspent_hash}}};

                        // Update the collection
                        match collection
                            .update_one(doc! {"wallet": wallet.to_string()}, update)
                            .await
                        {
                            Ok(_) => Ok(()),
                            Err(_) => Err("Error during the updating of utxos-(tools/utxo 67)"),
                        }
                    } else {
                        Err("UTXO does not exist!")
                    }
                }
                None => Err("UTXO does not exist!"),
            },
            Err(_) => Err("Problem from query of utxos collection-(tools/utxo 75)"),
        }
    }

    // Insert outputs of a transaction as UTXO
    pub async fn generate<'a>(
        block: u64,
        trx_hash: &String,
        output_hash: &String,
        unspent: &Unspent,
        db: &'a Database,
        wallet: &Public,
        window: &tauri::Window,
    ) -> Result<(), &'a str> {
        let utxo_wallet = unspent.data.wallet;
        // Create new UTXO
        let utxo = Self {
            block: block,
            trx_hash: trx_hash.to_string(),
            output_hash: output_hash.to_string(),
            unspent_hash: unspent.hash.to_string(),
            unspent: unspent.data.value,
        };
        let collection: Collection<Document> = db.collection("UTXOs");
        let query = collection
            .find_one(doc! {"wallet": utxo_wallet.to_string()})
            .await;
        match query {
            Ok(opt) => {
                if let Some(doc) = opt {
                    // Update existing person's UTXOs
                    let utxo_to_doc = to_document(&utxo).unwrap();
                    let update = doc! {"$push": {"utxos": utxo_to_doc}};
                    match collection.update_one(doc, update).await {
                        Ok(_) => {
                            if utxo_wallet == *wallet {
                                // Update sum of centies for the wallet
                                let sum_centies = Transactions::sum_centies(db, wallet).await;
                                match sum_centies {
                                    Ok(sum) => Ok(window.emit("sum_centies", sum).unwrap()),
                                    Err(e) => Err(e),
                                }
                            } else {
                                Ok(())
                            }
                        }
                        Err(_) => Err("Error while updating utxos-(tools/utxo 121)"),
                    }
                } else {
                    // Create new person with UTXO
                    let new_person = Person::new(utxo_wallet, vec![utxo]);
                    let person_to_doc = to_document(&new_person).unwrap();
                    match collection.insert_one(person_to_doc).await {
                        Ok(_) => {
                            if utxo_wallet == *wallet {
                                // Update sum of centies for the wallet
                                let sum_centies = Transactions::sum_centies(db, wallet).await;
                                match sum_centies {
                                    Ok(sum) => Ok(window.emit("sum_centies", sum).unwrap()),
                                    Err(e) => Err(e),
                                }
                            } else {
                                Ok(())
                            }
                        }
                        Err(_) => Err("Error while inserting utxos-(tools/utxo 140)"),
                    }
                }
            }
            Err(_) => Err("Error during query of mongodb-(tools/utxo 144)"),
        }
    }
}
