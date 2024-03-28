use crate::regex_state::RegexState;
#[derive(Debug)]
pub struct EvaluatedStep {
    pub state: RegexState,
    pub match_size: usize,
    pub backtrackable: bool,
}