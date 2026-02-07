use crate::state::State;
use rand::Rng;
use std::thread;
use std::time::Duration;
pub struct Node {
    pub id: u64,
    pub state: State,
    pub current_term: u64,
    pub voted_for: Option<u64>,
    pub log: Vec<LogEntry>,
    pub commit_index: u64,
}
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub term: u64,
    pub command: String,
}

impl Node {
    pub fn new(id: u64) -> Self {
        Node {
            id,
            state: State::Follower,
            current_term: 0,
            voted_for: None,
            log: Vec::new(),
            commit_index: 0,
        }
    }
    pub fn become_candidate(&mut self) {
        self.current_term += 1;
        self.state = State::Candidate;
        self.voted_for = Some(self.id);
    }
    pub fn start_election(&mut self) {
        let mut rng = rand::thread_rng();
        let timeout = rng.gen_range(150..300);
        thread::sleep(Duration::from_millis(timeout));
        self.become_candidate();
    }

    pub fn request_vote(&self, candidate_term: u64) -> bool {
        if candidate_term >= self.current_term {
            true
        } else {
            false
        }
    }
    pub fn become_leader(&mut self) {
        self.state = State::Leader;
    }

    pub fn receive_heartbeat(&mut self, leader_term: u64) {
        if leader_term >= self.current_term {
            self.state = State::Follower;
            self.current_term = leader_term;
        }
    }
    pub fn append_entry(&mut self, command: String) {
        if let State::Leader = self.state {
            let entry = LogEntry {
                term: self.current_term,
                command,
            };

            self.log.push(entry);
        }
    }
    pub fn replicate_entry(&mut self, entry: LogEntry, leader_term: u64) {
        if leader_term >= self.current_term {
            self.current_term = leader_term;
            self.state = State::Follower;

            self.log.push(entry);
        }
    }
    pub fn replicate_to_cluster(&self, cluster: &mut Vec<Node>) {
        for node in cluster.iter_mut() {
            if node.id != self.id {
                for entry in &self.log {
                    node.replicate_entry(entry.clone(), self.current_term);
                }
            }
        }
    }
}
