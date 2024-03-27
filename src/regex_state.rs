use crate::regex_value::RegexVal;
use crate::regex_rep::RegexRep;

#[derive(Debug)]
pub struct RegexState{
    pub value: RegexVal,
    pub repetition: RegexRep,
}