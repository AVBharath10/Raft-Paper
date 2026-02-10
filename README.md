# Raft Consensus Algorithm – Rust Implementation

A simplified implementation of the Raft consensus algorithm in Rust, demonstrating leader election, log replication, and heartbeats. This is a paper/learning implementation and not a production-ready system.

## Implemented Features

- Leader election with majority voting
- Log replication from leader to followers
- Heartbeat mechanism
- Randomized election timeouts
- State transitions: Follower, Candidate, Leader
- Basic Raft safety guarantees
- Test suite for core functionality

## Running

```bash
cargo run
cargo test
cargo build
```

## What This Demonstrates

- How elections are triggered and leaders are chosen
- How logs are replicated across nodes
- How AppendEntries ensures log consistency
- How heartbeats maintain leadership
- How terms control leader changes

### Project Structure

```
src/
├── main.rs              # Demo application and usage examples
├── node.rs              # Node implementation with Raft protocol
├── state.rs             # Node state enumeration
├── lib.rs               # Library module definitions and exports
└── tests/
    └── integration_tests.rs  # Comprehensive test suite
```

## Reference

Based on the Raft paper:
["In Search of an Understandable Consensus Algorithm"](https://raft.github.io/raft.pdf) – Diego Ongaro, John Ousterhout
