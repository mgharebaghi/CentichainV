use std::str::FromStr;

use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{
    events::db::DatabseConnection,
    generator::{relay::Relay, transaction::Transaction},
    tools::utxo::UTXO,
};

//response structure for request ustxo for a new transaction
#[derive(Debug, Serialize, Deserialize)]
pub struct ResBody {
    pub public_key: String,
    pub utxo_data: Vec<UTXO>,
    pub status: String,
    pub description: String,
}
//response structure for request ustxo for a new transaction
#[derive(Debug, Serialize, Deserialize)]
pub struct ReqBody {
    pub public_key: String,
    pub request: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct ResponseToFront {
    pub status: String,
    pub description: String,
}

#[tauri::command]
pub async fn send_transaction(
    wallet: String,
    private: String,
    to: String,
    value: String,
) -> ResponseToFront {
    match DatabseConnection::connect().await {
        Ok(db) => {
            match Relay::ip_adress(&db).await {
                Ok(ip) => {
                    let wallet = wallet.trim();
                    //conver str value to decimal and calculate fee
                    let decimal_value = Decimal::from_str(&value).unwrap();
                    let fee = decimal_value * Decimal::from_str("0.01").unwrap();

                    ////define request body for got input utxos
                    let client = Client::new();
                    let request = ReqBody {
                        public_key: wallet.to_string(),
                        request: "utxo".to_string(),
                        value,
                    };
                    let req_url = format!("http://{}:33369/autxo", ip);

                    //post request
                    match client.post(req_url).json(&request).send().await {
                        Ok(res) => {
                            let response: ResBody = res.json().await.unwrap();
                            if response.status == "success".to_string() {
                                match Transaction::new(
                                    wallet.to_string(),
                                    private,
                                    decimal_value,
                                    to,
                                    response,
                                    fee,
                                    client,
                                    ip,
                                )
                                .await
                                {
                                    Ok(_) => ResponseToFront {
                                        status: "success".to_string(),
                                        description: "Transaction Successfully Sent".to_string(),
                                    },
                                    Err(e) => ResponseToFront {
                                        status: "error".to_string(),
                                        description: e.to_string(),
                                    },
                                }
                            } else {
                                ResponseToFront {
                                    status: "error".to_string(),
                                    description: response.description,
                                }
                            }
                        }
                        Err(e) => ResponseToFront {
                            status: "error".to_string(),
                            description: e.to_string(),
                        },
                    }
                }
                Err(e) => ResponseToFront {
                    status: "error".to_string(),
                    description: e.to_string(),
                },
            }
        }
        Err(e) => ResponseToFront {
            status: "error".to_string(),
            description: e.to_string(),
        },
    }
}
