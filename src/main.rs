use raft::node::{Node, RequestVoteArgs};

fn main() {
    let mut nodes = vec![Node::new(1), Node::new(2), Node::new(3)];

    nodes[0].start_election();

    println!("Node1 state: {:?}", nodes[0].state);

    let term = nodes[0].current_term;

    let mut votes = 1;

    for i in 1..nodes.len() {
        let args = RequestVoteArgs {
            term,
            candidate_id: nodes[0].id,
            last_log_index: nodes[0].log.len() as u64,
            last_log_term: nodes[0].log.last().map(|e| e.term).unwrap_or(0),
        };

        let reply = nodes[i].request_vote(args);

        if reply.vote_granted {
            votes += 1;
        }
    }

    println!("Total votes: {}", votes);

    if votes > nodes.len() / 2 {
        nodes[0].become_leader();
        println!("Node 1 becomes LEADER");
        nodes[0].append_entry("set x=10".to_string());
        nodes[0].append_entry("set y=20".to_string());

        // Fix borrow checker issue by using separate mutable references
        let leader_id = nodes[0].id;
        let leader_term = nodes[0].current_term;
        let leader_log = nodes[0].log.clone();

        for node in nodes.iter_mut() {
            if node.id != leader_id {
                for entry in &leader_log {
                    node.replicate_entry(entry.clone(), leader_term);
                }
            }
        }
        for n in &nodes {
            println!("Node {} log: {:?}", n.id, n.log);
        }

        println!("Leader log: {:?}", nodes[0].log);

        let term = nodes[0].current_term;
        for i in 1..nodes.len() {
            nodes[i].receive_heartbeat(term);
        }
    } else {
        println!("Election failed");
    }
}
