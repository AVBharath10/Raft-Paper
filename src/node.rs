use crate::state::State;
use rand::Rng;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Node {
    pub id: u64,
    pub state: State,
    pub current_term: u64,
    pub voted_for: Option<u64>,
    pub log: Vec<LogEntry>,
    pub commit_index: u64,
    pub last_contact: Option<Instant>,

    pub next_index: std::collections::HashMap<u64, u64>,
    pub match_index: std::collections::HashMap<u64, u64>,
}

#[derive(Debug, Clone)]
pub struct AppendEntriesArgs {
    pub term: u64,
    pub leader_id: u64,

    pub prev_log_index: u64,
    pub prev_log_term: u64,

    pub entries: Vec<LogEntry>,

    pub leader_commit: u64,
}

#[derive(Debug)]
pub struct AppendEntriesReply {
    pub term: u64,
    pub success: bool,
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
            last_contact: None,
            next_index: std::collections::HashMap::new(),
            match_index: std::collections::HashMap::new(),
        }
    }
    pub fn become_candidate(&mut self) {
        self.current_term += 1;
        self.state = State::Candidate;
        self.voted_for = Some(self.id);
        self.last_contact = None;
    }
    pub fn start_election(&mut self) {
        self.become_candidate();
    }

    pub fn check_election_timeout(&mut self) -> bool {
        if self.state != State::Follower {
            return false;
        }

        let timeout_duration = Duration::from_millis(rand::thread_rng().gen_range(150..300));

        if let Some(last_contact) = self.last_contact {
            last_contact.elapsed() > timeout_duration
        } else {
            true
        }
    }

    pub fn send_heartbeat(&mut self, cluster: &mut Vec<Node>) {
        if self.state != State::Leader {
            return;
        }

        for node in cluster.iter_mut() {
            if node.id != self.id {
                let args = AppendEntriesArgs {
                    term: self.current_term,
                    leader_id: self.id,
                    prev_log_index: self.last_log_index(),
                    prev_log_term: self.last_log_term(),
                    entries: Vec::new(),
                    leader_commit: self.commit_index,
                };

                let reply = node.append_entries(args);
                if reply.term > self.current_term {
                    self.current_term = reply.term;
                    self.state = State::Follower;
                    self.voted_for = None;
                }
            }
        }
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

    pub fn become_leader(&mut self, cluster: &Vec<Node>) {
        self.state = State::Leader;

        self.next_index.clear();
        self.match_index.clear();

        let next = self.last_log_index() + 1;

        for node in cluster {
            if node.id != self.id {
                self.next_index.insert(node.id, next);
                self.match_index.insert(node.id, 0);
            }
        }
    }

    pub fn receive_heartbeat(&mut self, leader_term: u64) {
        if leader_term >= self.current_term {
            self.state = State::Follower;
            self.current_term = leader_term;
            self.last_contact = Some(Instant::now());
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

    pub fn send_append_entries(&mut self, node: &mut Node) {
        let next = *self.next_index.get(&node.id).unwrap();

        let prev_index = if next == 0 { 0 } else { next - 1 };

        let prev_term = if prev_index == 0 {
            0
        } else {
            self.log[prev_index as usize].term
        };

        let entries: Vec<LogEntry> = self.log.iter().skip(next as usize).cloned().collect();

        let args = AppendEntriesArgs {
            term: self.current_term,
            leader_id: self.id,
            prev_log_index: prev_index,
            prev_log_term: prev_term,
            entries,
            leader_commit: self.commit_index,
        };

        let reply = node.append_entries(args);

        if reply.success {
            let new_next = self.last_log_index() + 1;
            self.next_index.insert(node.id, new_next);
            self.match_index.insert(node.id, self.last_log_index());
        } else {
            let n = self.next_index.get(&node.id).unwrap();
            if *n > 0 {
                self.next_index.insert(node.id, n - 1);
            }
        }
    }

    pub fn replicate_to_cluster(&mut self, cluster: &mut Vec<Node>) {
        for node in cluster.iter_mut() {
            if node.id != self.id {
                self.send_append_entries(node);
            }
        }
        self.try_commit(cluster);
    }

    pub fn try_commit(&mut self, cluster: &Vec<Node>) {
        if self.state != State::Leader {
            return;
        }

        let mut n = self.commit_index + 1;
        while n <= self.last_log_index() {
            let mut count = 1; // leader itself

            for node in cluster {
                if node.id != self.id {
                    if let Some(&match_idx) = self.match_index.get(&node.id) {
                        if match_idx >= n {
                            count += 1;
                        }
                    }
                }
            }

            if count > cluster.len() / 2 {
                // Commit the entry
                self.commit_index = n;
                n += 1;
            } else {
                break;
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
    pub fn append_entries(&mut self, args: AppendEntriesArgs) -> AppendEntriesReply {
        if args.term < self.current_term {
            return AppendEntriesReply {
                term: self.current_term,
                success: false,
            };
        }

        if args.term > self.current_term {
            self.current_term = args.term;
            self.state = State::Follower;
            self.voted_for = None;
        }

        self.last_contact = Some(Instant::now());

        if args.prev_log_index > 0 {
            if (args.prev_log_index as usize) >= self.log.len() {
                return AppendEntriesReply {
                    term: self.current_term,
                    success: false,
                };
            }

            let my_term = self.log[args.prev_log_index as usize].term;

            if my_term != args.prev_log_term {
                self.log.truncate(args.prev_log_index as usize);

                return AppendEntriesReply {
                    term: self.current_term,
                    success: false,
                };
            }
        }

        for entry in args.entries {
            self.log.push(entry);
        }

        if args.leader_commit > self.commit_index {
            self.commit_index = std::cmp::min(args.leader_commit, self.last_log_index());
        }

        AppendEntriesReply {
            term: self.current_term,
            success: true,
        }
    }
}
