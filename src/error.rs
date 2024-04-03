use std::fmt;

#[derive(Debug, PartialEq)]
pub enum RegexError {
    UnmatchedBracket,
    NotInputFile,
    InvalidCharacterClassName,
    InvalidRegularExpression,
    NonAsciiInput,
    InvalidFile,
}

impl fmt::Display for RegexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnmatchedBracket => write!(f, "Unmatched [, [^, [:, [., or [="),
            Self::NotInputFile => write!(f, "Not an input file"),
            Self::InvalidCharacterClassName => write!(f, "Invalid character class name"),
            Self::InvalidRegularExpression => write!(f, "Invalid regular expression"),
            Self::NonAsciiInput => write!(f, "Input is not ASCII"),
            Self::InvalidFile => write!(f, "Invalid file"),
        }
    }
}
