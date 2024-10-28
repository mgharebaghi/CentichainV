use mongodb::{
    bson::{doc, from_document, Document},
    Collection,
};
use rust_decimal::Decimal;

use crate::{events::db::DatabseConnection, tools::utxo::Person};

#[tauri::command]
pub async fn sum_centies(wallet: String) -> String {
    match DatabseConnection::connect().await {
        Ok(db) => {
            let collection: Collection<Document> = db.collection("UTXOs");
            let filter = doc! {"wallet": wallet};
            let query = collection.find_one(filter).await;
            if let Ok(Some(doc)) = query {
                let person: Person = from_document(doc).unwrap();
                let sum: Decimal = person.utxos.iter().map(|utxo| utxo.unspent).sum();
                sum.to_string()
            } else {
                "0.0".to_string()
            }
        }
        Err(e) => e.to_string(),
    }
}
