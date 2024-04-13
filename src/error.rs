use std::fmt;

#[derive(Debug, PartialEq)]
/// Errors that can occur when parsing a regular expression
pub enum RegexError {
    /// The expression contains an unmatched bracket
    UnmatchedBracket,
    /// The expression contains an invalid character class name
    InvalidCharacterClassName,
    /// The expression is invalid
    InvalidRegularExpression,
    /// The input is not ASCII
    NonAsciiInput,
    /// The range in the bracket is invalid
    InvalidBracketRange,
    /// The file is invalid
    InvalidFile,
}

impl fmt::Display for RegexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnmatchedBracket => write!(f, "Unmatched [, [^, [:, [., or [="),
            Self::InvalidCharacterClassName => write!(f, "Invalid character class name"),
            Self::InvalidRegularExpression => write!(f, "Invalid regular expression"),
            Self::NonAsciiInput => write!(f, "Input is not ASCII"),
            Self::InvalidFile => write!(f, "Invalid file"),
            Self::InvalidBracketRange => write!(f, "Invalid range end"),
        }
    }
}
