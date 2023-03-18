// put consensus comamnds and data structures here.
// this is for terms, leader elections, etc.

use std::time::{Duration, SystemTime};

pub struct Entry {
    key: String,
    value: String,
}

pub enum ServerCommand {
    AppendEntries(Box<[Entry]>),
    RequestVote,
}

#[derive(Debug, PartialEq)]
pub enum ServerState {
    Leader,
    Follower,
}

#[derive(Debug)]
pub struct ServerId {
    id: i32,
    address: String,
}

#[derive(Debug)]
pub struct TermState {
    current_term: i128,
    server_state: ServerState,
    last_heartbeat: SystemTime,
    leader: Option<ServerId>,
    interval: u64, // raft heartbeat in ms
    servers: Vec<ServerId>,
}


impl TermState {
    pub fn new() -> TermState {
        TermState {
            current_term: 0,
            server_state: ServerState::Follower,
            last_heartbeat: SystemTime::UNIX_EPOCH,
            leader: None,
            interval: 5000,
            servers: vec![]
        }
    }

    pub async fn heartbeat(&mut self) -> Result<(), String> {
        println!("heartbeat loop start");
        let mut interval = tokio::time::interval(Duration::from_millis(self.interval));

        loop {
            interval.tick().await;
            self.heartbeat_action().await;
        }
    }

    async fn heartbeat_action(&mut self) {
        // do the things here
        println!("thump-thump");
        let now = SystemTime::now();

        // the algorithm:

        // IF this server is not the leader
        //   AND this server has not gotten a ping from the leader (within random timeout)

        if self.server_state == ServerState::Follower {
            if self.last_heartbeat.elapsed().unwrap() > Duration::from_millis(250) {

            // THEN 
            //   - change state to candidate  
            //   - increase term counter
            //   - vote for yourself
            //   - request votes from known servers

                self.request_vote();
            }
        }


      
    }

    fn request_vote(&mut self) {
      //  todo!("implement request vote.");
        println!("VOTE NOW WOOOO");
        for s in &self.servers {
            let addr = &s.address;
        }
    }

    // // i don't know if this should go here :(
    // fn create_clients(&mut self) {
    //     for s in &self.servers {
    //         let socket = 
    //     }
    // }
}
