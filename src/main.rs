use raft::{Node, RequestVoteArgs};

fn main() {
    let mut nodes = vec![Node::new(1), Node::new(2), Node::new(3)];

    println!("=== Raft Implementation Demo ===");

    // Simulate initial election
    nodes[0].start_election();
    println!("Node 1 started election, state: {:?}", nodes[0].state);

    let term = nodes[0].current_term;
    let mut votes = 1;

    // Request votes from other nodes
    for i in 1..nodes.len() {
        let args = RequestVoteArgs {
            term,
            candidate_id: nodes[0].id,
            last_log_index: nodes[0].last_log_index(),
            last_log_term: nodes[0].last_log_term(),
        };

        let reply = nodes[i].request_vote(args);

        if reply.vote_granted {
            votes += 1;
            println!("Node {} voted for Node 1", nodes[i].id);
        }
    }

    println!("Total votes for Node 1: {}", votes);

    if votes > nodes.len() / 2 {
        // Node 1 becomes leader - create a snapshot for become_leader
        let cluster_snapshot = nodes.clone();
        nodes[0].become_leader(&cluster_snapshot);
        println!("Node 1 becomes LEADER in term {}", nodes[0].current_term);

        // Add some log entries
        nodes[0].append_entry("set x = 10".to_string());
        nodes[0].append_entry("set y = 20".to_string());
        println!("Leader added 2 log entries");

        // Replicate log to followers
        println!("\n=== Log Replication ===");

        // Extract leader data
        let leader_id = nodes[0].id;
        let leader_term = nodes[0].current_term;
        let leader_commit = nodes[0].commit_index;
        let leader_log = nodes[0].log.clone();

        // Replicate to each follower
        for i in 0..nodes.len() {
            if nodes[i].id != leader_id {
                let args = raft::node::AppendEntriesArgs {
                    term: leader_term,
                    leader_id,
                    prev_log_index: 0,
                    prev_log_term: 0,
                    entries: leader_log.clone(),
                    leader_commit,
                };

                let reply = nodes[i].append_entries(args);
                if reply.success {
                    println!("Successfully replicated log to Node {}", nodes[i].id);
                    // Update commit index for the follower
                    if !leader_log.is_empty() {
                        nodes[i].commit_index = (leader_log.len() - 1) as u64;
                    }
                } else {
                    println!("Failed to replicate log to Node {}", nodes[i].id);
                }
            }
        }

        // Show logs after replication
        for node in &nodes {
            println!("Node {} log: {:?}", node.id, node.log);
            println!("Node {} commit_index: {}", node.id, node.commit_index);
            println!("Node {} state: {:?}", node.id, node.state);
        }

        // Send heartbeats
        println!("\n=== Heartbeat Mechanism ===");
        let heartbeat_args = raft::node::AppendEntriesArgs {
            term: leader_term,
            leader_id,
            prev_log_index: leader_log.len() as u64 - 1,
            prev_log_term: if leader_log.is_empty() {
                0
            } else {
                leader_log.last().unwrap().term
            },
            entries: Vec::new(),
            leader_commit: leader_commit,
        };

        for i in 0..nodes.len() {
            if nodes[i].id != leader_id {
                let reply = nodes[i].append_entries(heartbeat_args.clone());
                if reply.success {
                    println!("Heartbeat sent to Node {}", nodes[i].id);
                }
            }
        }
        println!("Leader sent heartbeats to all followers");

        // Simulate election timeout scenario
        println!("\n=== Election Timeout Test ===");
        let leader_term = nodes[0].current_term;
        for i in 1..nodes.len() {
            nodes[i].receive_heartbeat(leader_term);
        }
        println!("Followers received heartbeat, updated last_contact");

        // Test election timeout check (this would normally be in a loop)
        for i in 1..nodes.len() {
            let timeout = nodes[i].check_election_timeout();
            println!("Node {} election timeout: {}", nodes[i].id, timeout);
        }

        // Show final state
        println!("\n=== Final Cluster State ===");
        for node in &nodes {
            println!(
                "Node {}: term={}, state={}, log_len={}, commit_index={}",
                node.id,
                node.current_term,
                node.state,
                node.log.len(),
                node.commit_index
            );
        }
    } else {
        println!("Election failed - no majority reached");
    }

    println!("\n=== Demo Complete ===");
}
