use libp2p::PeerId;
use mongodb::{
    bson::{to_document, Document},
    Collection, Database,
};
use serde::{Deserialize, Serialize};
use sp_core::{ed25519::Public, Pair};

use crate::{
    generator::{relay::Relay, transaction::Transaction, HashMaker},
    tools::{
        trun_sync::{Sync, Turn},
        utxo::UTXO,
        waiting::Waiting,
    },
};

use super::{coinbase::Coinbase, header::Header};

// Define the structure of a block, including its header and body.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub header: Header,
    pub body: Body,
}

// Define the body of a block, which includes the coinbase transaction (encompassing rewards and fees)
// and other transactions that the block should have.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Body {
    pub coinbase: Coinbase,
    pub transactions: Vec<Transaction>,
}

impl Body {
    fn new(coinbase: Coinbase, transactions: Vec<Transaction>) -> Self {
        Self {
            coinbase,
            transactions,
        }
    }
}

impl Block {
    //make a new block
    //if transactions len was 0 then block generate a genesis block*********************************************************
    pub async fn new<'a>(
        db: &'a Database,
        transactions: Vec<Transaction>,
        wallet: &Public,
        peerid: &PeerId,
        private: &String,
        last_block: &mut Vec<Self>,
        relay: &mut Relay,
        window: &tauri::Window,
        turn: &mut Turn,
        sync_state: &mut Sync,
    ) -> Result<Self, &'a str> {
        //make coinbase transaction and if there is no ny problem make header and then return new block
        let coinbase = Coinbase::new(&transactions, wallet, last_block, relay).await;

        //make new body with coinebase that has maked and entrance transactons as parameter
        let body = Body::new(coinbase, transactions);

        //make header of block
        match Header::new(
            wallet,
            &body,
            peerid,
            &relay.peerid.unwrap(),
            private,
            last_block,
        )
        .await
        {
            Ok(header) => {
                let block = Self { header, body };

                //inseting new outputx as utxo into databse
                match Self::handle_outputs(&block, db, wallet, window).await {
                    //update waiting of validators
                    Ok(_) => match Waiting::update(db, peerid, turn, window, sync_state).await {
                        Ok(_) => {
                            //change last block
                            last_block.clear();
                            last_block.push(block.clone());

                            Ok(block)
                        }
                        Err(e) => Err(e),
                    },
                    Err(e) => Err(e),
                }
            }
            //return error if header had problem
            Err(e) => Err(e),
        }
    }

    //Block Validation*******************************************************************************
    pub async fn validation<'a>(
        &self,
        last_block: &mut Vec<Self>,
        db: &'a Database,
        mempool: &mut Vec<Transaction>,
        wallet: &Public,
        window: &tauri::Window,
        turn: &mut Turn,
        sync_state: &mut Sync,
    ) -> Result<Self, &'a str> {
        if last_block[0].header.hash == self.header.previous {
            //check block signature that its signature data is block hash(block hash is hash of body)
            let str_of_body = serde_json::to_string(&self.body).unwrap();
            let hash_of_body = HashMaker::generate(&str_of_body);
            let sign_check = sp_core::ed25519::Pair::verify(
                &self.header.signature.signatgure,
                hash_of_body,
                &self.header.signature.key,
            );

            //if block signature was correct then validation start validating of transactions in body of block
            //if found even 1 incorrect trx then block will be rejected
            if sign_check {
                //validate transactions in body
                let mut trx_err = None;
                let mut trx_backup: Vec<Transaction> = Vec::new();
                for i in 0..self.body.transactions.len() {
                    let index = mempool
                        .iter()
                        .position(|trx| trx.hash == self.body.transactions[i].hash);
                    //if trx was in mempool it means trx validated ans is correct so it removes from mempool
                    //if trx was not in mempool it validates the trx and if it doesn't have any problems will be accepted
                    if let Some(i) = index {
                        trx_backup.push(mempool.remove(i));
                    } else {
                        match Transaction::validate(&self.body.transactions[i], db).await {
                            Ok(_correcr) => {}
                            Err(e) => {
                                trx_err.get_or_insert(e);
                                break;
                            }
                        }
                    }
                }
                //if transactions of body doesn't have any problems then it goes to check coinbase transactions and insert utxos
                if trx_err.is_none() {
                    //validating coinbase of block and if it was correct then it will handle transactions of block
                    match Coinbase::validation(
                        &self.body.coinbase,
                        last_block,
                        &self.body.transactions,
                    )
                    .await
                    {
                        Ok(_) => {
                            //if generating utxo in database doesn't have any errors then set waiting of validators
                            //else return error of block
                            match Self::handle_outputs(&self, db, wallet, window).await {
                                Ok(_) => {
                                    //if updating waiting doesn't any problems return block as correct block
                                    //else return error of updating
                                    match Waiting::update(
                                        db,
                                        &self.header.validator,
                                        turn,
                                        window,
                                        sync_state,
                                    )
                                    .await
                                    {
                                        Ok(_) => {
                                            //change last block
                                            last_block.clear();
                                            last_block.push(self.clone());

                                            Ok(self.clone())
                                        }
                                        Err(e) => Err(e),
                                    }
                                }
                                Err(e) => {
                                    //if there is any true trx in the rejected block that it removes its input utxos from database then it returns the trx to mempool
                                    //before return an error about block rejection
                                    if trx_backup.len() > 0 {
                                        for t in trx_backup {
                                            mempool.push(t);
                                        }
                                    }
                                    Err(e)
                                }
                            }
                        }
                        Err(e) => {
                            //if there is any true trx in the rejected block that it removes its input utxos from database then it returns the trx to mempool
                            //before return an error about block rejection
                            if trx_backup.len() > 0 {
                                for t in trx_backup {
                                    mempool.push(t);
                                }
                            }
                            Err(e)
                        }
                    }
                } else {
                    //if there is any true trx in the rejected block that it removes its input utxos from database then it returns the trx to mempool
                    //before return an error about block rejection
                    if trx_backup.len() > 0 {
                        for t in trx_backup {
                            mempool.push(t);
                        }
                    }
                    Err(trx_err.unwrap())
                }
            } else {
                Err("Block signature is wrong and Block rejected.")
            }
        } else {
            Err("Block validation problem!, previous hash doesn't match and Block rejected.")
        }
    }

    //make and insert new utxos from outputs of coinbase and tranasactions into database*****************************************************
    async fn handle_outputs<'a>(
        &self,
        db: &'a Database,
        wallet: &Public,
        window: &tauri::Window,
    ) -> Result<(), &'a str> {
        let mut utxo_err: Option<&str> = None;
        //generate new utxo for each unspents of outputs of coinbase of recieved block
        for unspent in &self.body.coinbase.output.unspents {
            match UTXO::generate(
                self.header.number,
                &self.body.coinbase.hash,
                &self.body.coinbase.output.hash,
                unspent,
                db,
                wallet,
                window,
            )
            .await
            {
                Ok(_) => {}
                Err(e) => {
                    utxo_err.get_or_insert(e);
                    break;
                }
            }
        }

        //generate new utxo for each unspents of outputs of transactiosn of recieved block
        for trx in &self.body.transactions {
            for unspent in &trx.output.unspents {
                match UTXO::generate(
                    self.header.number,
                    &trx.hash,
                    &trx.output.hash,
                    unspent,
                    db,
                    wallet,
                    window,
                )
                .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        utxo_err.get_or_insert(e);
                        break;
                    }
                }
            }
        }

        match utxo_err {
            None => Ok(()),
            Some(e) => Err(e),
        }
    }

    //insert block to database
    pub async fn insertion<'a>(self, db: &'a Database) -> Result<(), &'a str> {
        let collection: Collection<Document> = db.collection("Blocks");
        let doc = to_document(&self).unwrap();

        match collection.insert_one(doc).await {
            Ok(_) => Ok(()),
            Err(_) => Err("Error during insertion of block-(generation/block/block 249)"),
        }
    }
}
