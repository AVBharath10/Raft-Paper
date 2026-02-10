#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Leader,
    Follower,
    Candidate,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Leader => write!(f, "Leader"),
            State::Follower => write!(f, "Follower"),
            State::Candidate => write!(f, "Candidate"),
        }
    }
}
