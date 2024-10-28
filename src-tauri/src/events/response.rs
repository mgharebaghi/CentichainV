use libp2p::{PeerId, Swarm};
use mongodb::Database;
use sp_core::ed25519::Public;

use crate::{
    generator::{
        block::{block::Block, message::BlockMessage},
        leader::Leader,
        relay::Relay,
        swarm::{CentichainBehaviour, Res},
        transaction::Transaction,
    },
    tools::trun_sync::{Sync, Turn},
};

use super::{handshaking::Handshake, syncing::VSync};

pub struct Responses;

impl Responses {
    pub async fn handle<'a>(
        window: &tauri::Window,
        response: Res,
        db: &'a Database,
        wallet: &'a Public,
        peerid: &'a PeerId,
        trun: &mut Turn,
        private: &'a String,
        mempool: &mut Vec<Transaction>,
        recieved_blocks: &mut Vec<BlockMessage>,
        last_block: &mut Vec<Block>,
        relay: &mut Relay,
        swarm: &mut Swarm<CentichainBehaviour>,
        sync_state: &mut Sync,
        leader: &mut Leader,
    ) -> Result<(), &'a str> {
        //check hadnshake response to start syncing or generate genesis block and set validator turn on
        //if
        match Handshake::response(
            window,
            response,
            &db,
            wallet,
            peerid,
            private,
            trun,
            mempool,
            recieved_blocks,
            last_block,
            relay,
            leader,
            swarm,
            sync_state,
        )
        .await
        {
            Ok(_) => {
                let sync = VSync::new(
                    &relay.peerid.unwrap(),
                    peerid,
                    "I'm Synced".to_string(),
                    wallet,
                );
                match sync.propagate(swarm, window) {
                    Ok(_) => Ok(sync_state.synced()),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }
}
