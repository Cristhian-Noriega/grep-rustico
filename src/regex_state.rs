use crate::regex_value::RegexVal;
use crate::regex_rep::RegexRep;

#[derive(Debug)]
pub struct RegexState{
    pub value: RegexVal,
    pub repetition: RegexRep,
}

impl RegexState {
    pub fn is_final(&self) -> bool {
        match self.repetition {
            RegexRep::Exact(n) => n == 1,
            _ => false,
        }
    }
}