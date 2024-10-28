use std::str::FromStr;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use sp_core::ed25519::Public;

use crate::generator::{
    relay::Relay,
    transaction::{Output, Transaction, Unspent},
    HashMaker, MerkelRoot,
};

use super::{block::Block, reward::Reward};

//fees are sum of transactions fees
//relay's fee is 10% of fees
//validator fee is 90% of fees
//reward is only for validator
//merkel is merkel root of transactions
//size is number of transactions
//hash make from output
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coinbase {
    pub hash: String,
    size: u8,
    pub merkel: String,
    #[serde_as(as = "DisplayFromStr")]
    pub reward: Decimal,
    pub output: Output,
    #[serde_as(as = "DisplayFromStr")]
    fees: Decimal,
    #[serde_as(as = "DisplayFromStr")]
    relay_fee: Decimal,
    #[serde_as(as = "DisplayFromStr")]
    validator_fee: Decimal,
}

impl Coinbase {
    pub async fn new<'a>(
        transactions: &Vec<Transaction>,
        wallet: &Public,
        last_block: &mut Vec<Block>,
        relay: &mut Relay,
    ) -> Self {
        let mut transactions_hash = Vec::new();
        let mut fees = Decimal::from_str("0.0").unwrap().round_dp(12);
        let mut relay_fee = Decimal::from_str("0.0").unwrap().round_dp(12);
        let mut validator_fee = Decimal::from_str("0.0").unwrap().round_dp(12);
        let mut merkel_root = "First".to_string();

        //sum of fees in transactions in the block if the block has transactions
        if transactions.len() > 0 {
            //calculate fees
            fees = transactions.iter().map(|trx| trx.fee).sum();

            //get hashes of transactions for make merkel root
            for trx in transactions {
                transactions_hash.push(&trx.hash);
            }
            //relay & validator's fee
            relay_fee = fees * Decimal::from_str("0.10").unwrap().round_dp(12);
            validator_fee = fees - relay_fee;

            //calculate merkel root of transactions
            merkel_root = MerkelRoot::make(transactions_hash).first().unwrap().clone();
        }

        //size of block
        let size = (&transactions.len() + 1) as u8;

        //calculate reward
        let reward = Reward::calculate(last_block);

        let mut outputs: Vec<Unspent> = Vec::new();

        let relay_wallet: Public = relay.wallet.parse().unwrap();

        //make outputs of coinbase transaction(if block was genesis it means fee is 0.0 and then coinbase pay rewards as two chunks)
        if fees == Decimal::from_str("0.0").unwrap().round_dp(12) {
            outputs.push(Unspent::new(
                wallet,
                reward / Decimal::from_str("2.0").unwrap().round_dp(12),
            ));
            outputs.push(Unspent::new(
                wallet,
                reward / Decimal::from_str("2.0").unwrap().round_dp(12),
            ));
        } else {
            outputs.push(Unspent::new(wallet, reward));
            outputs.push(Unspent::new(&relay_wallet, relay_fee));
            outputs.push(Unspent::new(wallet, validator_fee));
        }
        let output = Output::new(outputs);

        //define coinbase and return it
        let coinbase = Self {
            hash: HashMaker::generate(&output.hash),
            size,
            merkel: merkel_root,
            reward,
            output,
            fees,
            relay_fee,
            validator_fee,
        };
        coinbase
    }

    //validating coinbase trx in a block that recieve
    pub async fn validation<'a>(
        &self,
        last_block: &mut Vec<Block>,
        transactions: &Vec<Transaction>,
    ) -> Result<(), &'a str> {
        let reward = Reward::calculate(last_block);
        if self.reward == reward {
            //make merkel root of block's transactions
            let mut trx_hashes = Vec::new();
            for t in transactions {
                trx_hashes.push(&t.hash);
            }
            let merkel = MerkelRoot::make(trx_hashes).first().unwrap().clone();

            //check merkel root that maked with coinbase merkel root to validation
            if merkel == self.merkel {
                //calculate fees
                let fees: Decimal = transactions.iter().map(|trx| trx.fee).sum();
                let relay_fee = fees * Decimal::from_str("0.10").unwrap();
                let validator_fee = fees - relay_fee;

                //if fees was correct check outputs of coinbase for validate relay fee
                if fees == self.fees
                    && relay_fee == self.relay_fee
                    && validator_fee == self.validator_fee
                {
                    // if relay's fee was greater that 0 then check outputs of coinbase to see relay's fee
                    //if there is no any output for relay's fee coinbase will be rejected
                    if relay_fee > Decimal::from_str("0.0").unwrap() {
                        let mut relay_fee_check: Option<bool> = None;
                        for unspent in &self.output.unspents {
                            if unspent.data.value == relay_fee {
                                relay_fee_check.get_or_insert(true);
                            }
                        }
                        if relay_fee_check.unwrap() {
                            Ok(())
                        } else {
                            Err("Coinbase's relay fee is wrong!")
                        }
                    } else {
                        Ok(())
                    }
                } else {
                    Err("fees of coinbase transaction is wrong!")
                }
            } else {
                Err("Merkel root of coinbase is wrong!")
            }
        } else {
            Err("Recieved block's reward in coinbase is incorrect!")
        }
    }
}
