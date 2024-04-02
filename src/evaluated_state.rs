use crate::regex_state::RegexState;

/// An EvaluatedStep represents an state that has been already evaluated in the process of matching.
/// It contains the state itself, the size of the match, and a flag indicating if the state is backtrackable.
/// Backtrackable states are those that can be revisited in the future, cause they can be part of a valid match.
#[derive(Debug)]
pub struct EvaluatedStep {
    pub state: RegexState,
    pub match_size: usize,
    pub backtrackable: bool,
}
