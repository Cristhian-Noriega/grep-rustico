use crate::regex_rep::RegexRep;
use crate::regex_value::RegexVal;

/// Represents a single state in a regular expression.
/// A state consists of a value and a repetition specifier.
#[derive(Debug, Clone)]
pub struct RegexState {
    pub value: RegexVal,
    pub repetition: RegexRep,
}
