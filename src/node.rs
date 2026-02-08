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
#[derive(Debug)]
pub struct RequestVoteArgs {
    pub term: u64,
    pub candidate_id: u64,
    pub last_log_index: u64,
    pub last_log_term: u64,
}
#[derive(Debug)]
pub struct RequestVoteReply {
    pub term: u64,
    pub vote_granted: bool,
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

    pub fn request_vote(&mut self, args: RequestVoteArgs) -> RequestVoteReply {
        if args.term < self.current_term {
            return RequestVoteReply {
                term: self.current_term,
                vote_granted: false,
            };
        }

        if args.term > self.current_term {
            self.current_term = args.term;
            self.voted_for = None;
            self.state = State::Follower;
        }

        let not_voted_yet = self.voted_for.is_none() || self.voted_for == Some(args.candidate_id);

        let my_last_term = self.last_log_term();
        let my_last_index = self.last_log_index();

        let candidate_up_to_date = (args.last_log_term > my_last_term)
            || (args.last_log_term == my_last_term && args.last_log_index >= my_last_index);

        if not_voted_yet && candidate_up_to_date {
            self.voted_for = Some(args.candidate_id);

            RequestVoteReply {
                term: self.current_term,
                vote_granted: true,
            }
        } else {
            RequestVoteReply {
                term: self.current_term,
                vote_granted: false,
            }
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
    pub fn last_log_index(&self) -> u64 {
        if self.log.is_empty() {
            0
        } else {
            (self.log.len() - 1) as u64
        }
    }
    pub fn last_log_term(&self) -> u64 {
        if self.log.is_empty() {
            0
        } else {
            self.log.last().unwrap().term
        }
    }
}
