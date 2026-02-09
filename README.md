# Raft Consensus Algorithm Implementation

A simple implementation of the Raft consensus algorithm in Rust, demonstrating leader election, log replication, and basic consensus mechanisms

## Overview

This implementation showcases the core concepts of the Raft algorithm:

- **Leader Election**: Nodes can start elections and become leaders through voting
- **Log Replication**: Leaders replicate log entries to followers in the cluster
- **State Management**: Nodes transition between Follower, Candidate, and Leader states
- **Safety**: Ensures at most one leader per term and log consistency

## Features

- Multi-node cluster simulation
- Leader election with randomized timeouts
- Log replication across cluster members
- Heartbeat mechanism for leader maintenance
- Basic Raft state machine operations

## How It Works

1. **Initialization**: Creates a cluster of 3 nodes starting as Followers
2. **Election**: Node 1 initiates an election and becomes a Candidate
3. **Voting**: Other nodes vote for the candidate
4. **Leadership**: If majority votes received, candidate becomes Leader
5. **Replication**: Leader replicates log entries to all followers
6. **Heartbeat**: Leader sends heartbeats to maintain authority


## Project Structure

```
src/
├── main.rs      # Main application logic and demonstration
├── node.rs      # Node implementation with Raft protocol
├── state.rs     # Node state enumeration (Follower, Candidate, Leader)
└── lib.rs       # Library module definitions
```

## Dependencies

- `rand` 0.8.0 - For randomized election timeouts

## API Reference

### Node Methods

- `new(id: u64)` - Create a new node with given ID
- `start_election()` - Begin the leader election process
- `request_vote(candidate_term: u64) -> bool` - Handle vote requests
- `become_leader()` - Transition to leader state
- `append_entry(command: String)` - Add entry to leader's log
- `replicate_to_cluster(cluster: &mut Vec<Node>)` - Replicate log to followers
- `receive_heartbeat(leader_term: u64)` - Handle leader heartbeats

### States

- `Follower` - Default state, responds to leader requests
- `Candidate` - Contesting in an election
- `Leader` - Manages the cluster and handles client requests



This is a simplified implementation for educational purposes:

- No persistent storage (all state is in-memory)
- No client request handling
- No network layer (simulation only)
- No log compaction
- No configuration changes
- Basic safety properties only

## Future Enhancements

- Persistent log storage
- Network communication between nodes
- Client command interface
- Log compaction
- Cluster membership changes
- More robust testing

## Acknowledgments

Based on the original Raft paper: ["In Search of an Understandable Consensus Algorithm"](https://raft.github.io/raft.pdf) by Diego Ongaro and John Ousterhout.

---

<!-- View Counter -->
👥 **Views:** ![Views](https://api.visitorbadge.io/api/combined?user=your-username&repo=raft&labelColor=%23555555&countColor=%2300aaff&style=flat-square)

*Last updated: February 2026*
