use crate::regex_value::RegexVal;
use crate::regex_rep::RegexRep;

#[derive(Debug, Clone)]
pub struct RegexState{
    pub value: RegexVal,
    pub repetition: RegexRep,
}
