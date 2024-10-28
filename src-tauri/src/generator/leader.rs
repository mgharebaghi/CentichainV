use std::collections::HashMap;

use chrono::{DateTime, Duration, Utc};
use libp2p::{gossipsub::IdentTopic, PeerId, Swarm};
use mongodb::{
    bson::{doc, from_document, Document},
    Collection, Database,
};
use tauri::Emitter;

use crate::{
    events::gossip_messages::handler::GossipMessages,
    tools::{
        trun_sync::{Sync, Turn},
        wrongdoer::WrongDoer,
    },
};

use super::{swarm::CentichainBehaviour, validator::Validator};

pub struct Leader {
    pub peerid: Option<PeerId>,
    pub timer: LeaderTime,
    pub time: Option<DateTime<Utc>>,
    pub in_check: bool,
    pub votes: Vec<PeerId>,
}

#[derive(Debug, PartialEq)]
pub enum LeaderTime {
    On,
    Off,
}

impl LeaderTime {
    pub fn start(&mut self) {
        *self = Self::On
    }

    pub fn off(&mut self) {
        *self = Self::Off
    }
}

impl Leader {
    // Define new leader
    pub fn new(peerid: Option<PeerId>, window: &tauri::Window) -> Self {
        if peerid.is_some() {
            window.emit("leader", peerid.unwrap().to_string()).unwrap();
        }
        Self {
            peerid,
            timer: LeaderTime::Off,
            time: None,
            in_check: false,
            votes: Vec::new(),
        }
    }

    // Set in_check to true
    fn check_start(&mut self) {
        self.in_check = true
    }

    // Start leader time for checking its block
    pub fn timer_start(&mut self) {
        self.timer.start();
        // Set time to 59 seconds from now
        self.time.get_or_insert(Utc::now() + Duration::seconds(59));
    }

    // Update to new leader
    pub fn update(&mut self, peerid: Option<PeerId>, window: &tauri::Window) {
        self.timer.off();
        self.time = None;
        self.peerid = peerid;
        self.in_check = false;
        // Emit leader event if peerid is present
        if peerid.is_some() {
            window.emit("leader", peerid.unwrap().to_string()).unwrap();
        }
    }

    // Propagate validator's vote about new leader to the network
    pub async fn start_voting<'a>(
        &mut self,
        db: &'a Database,
        swarm: &mut Swarm<CentichainBehaviour>,
        peerid: &PeerId,
        window: &tauri::Window,
        turn: &mut Turn,
        sync_state: &mut Sync,
    ) -> Result<(), &'a str> {
        // Set in_check of leader to true
        self.check_start();

        // First, delete left leader from validators as a wrongdoer
        match WrongDoer::remove(db, self.peerid.unwrap().clone(), turn, sync_state, window).await {
            Ok(wrongdoer) => {
                window
                    .emit(
                        "status",
                        format!("Left leader remove as a wrongdoer: {}", wrongdoer),
                    )
                    .unwrap();

                // Propagate vote to the network
                self.find_and_post_new_leader(db, swarm, peerid, window, turn)
                    .await
            }
            Err(e) => Err(e),
        }
    }

    // Finding new leader for sending it as vote
    async fn find_and_post_new_leader<'a>(
        &mut self,
        db: &'a Database,
        swarm: &mut Swarm<CentichainBehaviour>,
        peerid: &PeerId,
        window: &tauri::Window,
        turn: &mut Turn,
    ) -> Result<(), &'a str> {
        let collection: Collection<Document> = db.collection("validators");
        // Finding validator that its waiting is 0
        let filter = doc! {"waiting": 0};
        let query = collection.find_one(filter).await;
        match query {
            Ok(opt) => match opt {
                // If there was a validator that its waiting is 0, post it as vote
                Some(doc) => {
                    let validator: Validator = from_document(doc).unwrap();
                    let vote = GossipMessages::LeaderVote(validator.peerid);
                    let str_vote = serde_json::to_string(&vote).unwrap();
                    match swarm
                        .behaviour_mut()
                        .gossipsub
                        .publish(IdentTopic::new("validator"), str_vote)
                    {
                        Ok(_) => Ok(window.emit("leader", validator.peerid.to_string()).unwrap()),
                        Err(_) => Err("Error while gossiping message-(generator/leader 120)"),
                    }
                }

                // If there isn't validator that its waiting was 0, post current validator own peerid as vote
                None => {
                    let vote = GossipMessages::LeaderVote(peerid.clone());
                    let str_vote = serde_json::to_string(&vote).unwrap();
                    match swarm
                        .behaviour_mut()
                        .gossipsub
                        .publish(IdentTopic::new("validator"), str_vote)
                    {
                        Ok(_) => {
                            self.update(Some(peerid.clone()), window);
                            turn.on(window);
                            Ok(window.emit("leader", peerid.to_string()).unwrap())
                        }
                        Err(_) => Err("Error while gossiping message-(generator/leader 137)"),
                    }
                }
            },
            Err(_) => {
                Err("Error during find validator that its waiting is 0-(generator/leader 142)")
            }
        }
    }

    // Check votes and if it was quorum set it as leader
    pub async fn check_votes<'a>(
        &mut self,
        db: &Database,
        vote: PeerId,
        peerid: &PeerId,
        turn: &mut Turn,
        window: &tauri::Window,
    ) -> Result<(), &'a str> {
        let collection: Collection<Document> = db.collection("validators");
        // Get count documents for knowing votes are upper than 50% of documents number or not
        match collection.count_documents(doc! {}).await {
            Ok(count) => {
                self.votes.push(vote);

                // If votes are upper than 50% of validators then find most vote to set as leader
                if self.votes.len() >= ((count / 2) + 1) as usize {
                    let mut hashmap_of_votes = HashMap::new(); // Make hashmap to group by vote per peerid
                    for v in self.votes.clone() {
                        *hashmap_of_votes.entry(v).or_insert(0) += 1; // Plus 1 if key is repetitive
                    }
                    // Get the most vote and change leader
                    let result = hashmap_of_votes
                        .iter()
                        .max_by_key(|v| *v)
                        .unwrap()
                        .0
                        .clone();
                    if &result == peerid {
                        self.update(Some(result), window);
                        turn.on(window);
                    } else {
                        self.update(Some(result), window);
                        self.timer_start();
                    }
                }
                Ok(())
            }
            Err(_) => Err("Error while get count of validators' doc-(generator/leader 184)"),
        }
    }
}
