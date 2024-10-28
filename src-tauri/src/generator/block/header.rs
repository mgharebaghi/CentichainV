use centichain_keypair::CentichainKey;
use chrono::{SubsecRound, Utc};
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use sp_core::ed25519::{Public, Signature};

use crate::generator::{HashMaker, MerkelRoot};

use super::block::{Block, Body};

// Define the structure of a block's header with signature.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Header {
    pub number: u64,
    pub hash: String,
    pub previous: String,
    pub validator: PeerId,
    pub relay: PeerId,
    merkel: String,
    pub signature: Sign,
    date: String,
}

// Define the structure of a signature, including the signature itself and the public key for verification.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sign {
    pub signatgure: Signature,
    pub key: Public,
}

impl Header {
    pub async fn new<'a>(
        wallet: &Public,
        body: &Body,
        peerid: &PeerId,
        relay_id: &PeerId,
        private: &String,
        last_block: &mut Vec<Block>,
    ) -> Result<Self, &'a str> {
        //define fields
        let number: u64;
        let hash_data = serde_json::to_string(body).unwrap();
        let hash = HashMaker::generate(&hash_data);
        let mut previous = String::new();
        let trx_hashes = vec![&body.coinbase.hash, &body.coinbase.merkel];
        let merkel = MerkelRoot::make(trx_hashes).first().unwrap().clone();

        if last_block.len() > 0 {
            number = last_block[0].header.number + 1;
            previous.push_str(&last_block[0].header.hash);
            let sign_block = CentichainKey::signing(private, &hash);
            match sign_block {
                Ok(signed) => {
                    let signature = Sign {
                        signatgure: signed,
                        key: *wallet,
                    };
                    let date = Utc::now().round_subsecs(0).to_string();
                    Ok(Self {
                        number,
                        hash,
                        previous,
                        validator: *peerid,
                        relay: *relay_id,
                        merkel,
                        signature,
                        date,
                    })
                }
                Err(_e) => Err("Error during signing-(generator/header 82)"),
            }
        } else {
            number = 1;
            previous.push_str("This Is The Genesis Block");
            let sign_block = CentichainKey::signing(private, &hash);
            match sign_block {
                Ok(signed) => {
                    let signature = Sign {
                        signatgure: signed,
                        key: *wallet,
                    };
                    let date = Utc::now().round_subsecs(0).to_string();
                    Ok(Self {
                        number,
                        hash,
                        previous,
                        validator: *peerid,
                        relay: *relay_id,
                        merkel,
                        signature,
                        date,
                    })
                }
                Err(_e) => Err("Error during signing-(generator/header 94)"),
            }
        }
    }
}
