use chrono::{SubsecRound, Utc};
use mongodb::Database;
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use sp_core::{crypto::Ss58Codec, ed25519::Public, Pair};

use crate::tools::{for_front::make_trx::ResBody, utxo::UTXO};

use super::{block::header::Sign, HashMaker, MerkelRoot};

// Define a transaction in the Centichain network
// The hash of the transaction is derived from the hashes of its inputs and outputs
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub hash: String,
    pub input: Input,
    pub output: Output,
    #[serde_as(as = "DisplayFromStr")]
    pub value: Decimal,
    #[serde_as(as = "DisplayFromStr")]
    pub fee: Decimal,
    pub script: Script,
    pub signature: Vec<Sign>,
    pub date: String,
}

// Define a script for highlighting the transaction's signature
// It can have either a single signature or multiple signatures
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Script {
    Single,
    Multi,
}

// Define an input that includes UTXOs from other transactions' outputs, the number of UTXOs,
// and the hash of the UTXOs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Input {
    hash: String,
    number: u8,
    utxos: Vec<UTXO>,
}

// Define an output that includes new UTXOs, the number of UTXOs, and the public key of the transaction creator
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Output {
    pub hash: String,
    pub number: usize,
    pub unspents: Vec<Unspent>,
}

//Define an unspent that has hash of unpsnet which is a new output for make a special UTXO
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Unspent {
    pub hash: String,
    pub data: UnspentData,
}
//Define a new output of a transaction that includes the wallet address,
// a salt for better hashing, and the unspent value derived from the transaction input
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnspentData {
    pub wallet: Public,
    pub salt: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub value: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxRes {
    pub hash: String,
    pub status: String,
    pub description: String,
}

impl Input {
    fn new(response: ResBody) -> Self {
        let hash_data = serde_json::to_string(&response.utxo_data).unwrap();
        let hash = HashMaker::generate(&hash_data);
        Self {
            hash,
            number: response.utxo_data.len() as u8,
            utxos: response.utxo_data,
        }
    }
}

impl Output {
    pub fn new(unspents: Vec<Unspent>) -> Self {
        let str_outputs = serde_json::to_string(&unspents).unwrap();
        let output = Self {
            hash: HashMaker::generate(&str_outputs),
            number: unspents.len(),
            unspents,
        };

        output
    }
}

impl Unspent {
    pub fn new<'a>(wallet: &Public, value: Decimal) -> Self {
        let salt: u32 = rand::random();
        let data = UnspentData {
            wallet: *wallet,
            salt,
            value,
        };

        let hash_data = serde_json::to_string(&data).unwrap();

        Self {
            hash: HashMaker::generate(&hash_data),
            data,
        }
    }
}

impl Transaction {
    pub async fn validate<'a>(&self, db: &Database) -> Result<bool, &'a str> {
        //make input and output hash to check hash that is correct or not
        let inputs_str = serde_json::to_string(&self.input.utxos).unwrap();
        let outputs_str = serde_json::to_string(&self.output.unspents).unwrap();
        let input_hash = HashMaker::generate(&inputs_str);
        let output_hash = HashMaker::generate(&outputs_str);

        //check input and output hash that is correct or not
        if input_hash == self.input.hash && output_hash == self.output.hash {
            //make tansaction's hash for check that it is correct or not
            let hashes = vec![&input_hash, &output_hash];
            let trx_hash = MerkelRoot::make(hashes);

            //check transaction hash
            if trx_hash[0] == self.hash {
                //validating signatrue of trx
                let sign_check = sp_core::ed25519::Pair::verify(
                    &self.signature[0].signatgure,
                    &trx_hash[0],
                    &self.signature[0].key,
                );

                //if validation done transaction is correct
                if sign_check {
                    //validating input utxos
                    let mut is_err: Option<&str> = None;
                    for i in 0..self.input.utxos.len() {
                        match UTXO::check(&self.input.utxos[i], db, &self.signature[0].key).await {
                            Ok(_) => {}
                            Err(e) => {
                                is_err = Some(e);
                                break;
                            }
                        }
                    }

                    //if inputs utxo doesn't have any problems return true
                    if is_err.is_none() {
                        Ok(true)
                    } else {
                        Err(is_err.unwrap())
                    }
                } else {
                    Err("Transaction is incorrect.(siganture problem!)")
                }
            } else {
                Err("Transaction is incorrect.(transacrtion hash problem!)")
            }
        } else {
            Err("Transaction is incorrect.(input/output hash problem!)")
        }
    }

    //make new transaction and send it to relay
    pub async fn new<'a>(
        public_key: String,
        private_key: String,
        value: Decimal,
        to: String,
        response: ResBody,
        fee: Decimal,
        client: Client,
        ip: String,
    ) -> Result<(), &'a str> {
        let wallet = sp_core::ed25519::Public::from_string(&public_key).unwrap();
        let sum_input: Decimal = response
            .utxo_data
            .iter()
            .map(|unspent| unspent.unspent)
            .sum();

        //make transaction after get resposne
        let input = Input::new(response); //make input
        let mut unspents = Vec::new();
        let change_wallet: Public = public_key.parse().unwrap();

        match to.parse() {
            Ok(to_wallet) => {
                //calculat change
                if sum_input > value + fee {
                    let change = sum_input - (value + fee);
                    unspents.push(Unspent::new(&change_wallet, change));
                    unspents.push(Unspent::new(&to_wallet, value));
                } else {
                    unspents.push(Unspent::new(&to_wallet, value));
                }
                let output = Output::new(unspents); // make output

                //make transaction hash and signature
                let hash = MerkelRoot::make(vec![&input.hash, &output.hash]);
                let sign =
                    centichain_keypair::CentichainKey::signing(&private_key, &hash[0]).unwrap();
                let signature = Sign {
                    signatgure: sign,
                    key: wallet,
                };

                //define transaction
                let transaction = Self {
                    hash: hash[0].clone(),
                    input,
                    output,
                    value,
                    fee,
                    script: Script::Single,
                    signature: vec![signature],
                    date: Utc::now().round_subsecs(0).to_string(),
                };

                let url = format!("http://{}:33369/trx", ip);

                match client.post(url).json(&transaction).send().await {
                    Ok(res) => {
                        let trx_res: TxRes = res.json().await.unwrap();
                        if trx_res.status == "success".to_string() {
                            Ok(())
                        } else {
                            Err("server has problem! please try with another provider.")
                        }
                    }
                    Err(_e) => Err("Post transaction problem!"),
                }
            }
            Err(_) => Err("Wallet address is incorrect!"),
        }
    }
}
