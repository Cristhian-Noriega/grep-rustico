use crate::regex_class::RegexClass;

/// Represents a single value in a regular expression.

#[derive(Debug, Clone)]
pub enum RegexVal {
    /// A literal character value with no special meaning.
    Literal(char),
    /// A wildcard value that matches any single character.
    Wildcard,
    /// A bracket expression that matches any single character in the provided list.
    /// If `is_negated` is `true`, the expression matches any character not in the list.
    BracketExpression { chars: Vec<char>, is_negated: bool },
    /// A character class with an enum RegexClass as value.
    Class(RegexClass),
}

impl RegexVal {
    /// Determines if the value matches the beginning of a string.
    ///
    /// # Arguments
    ///
    /// * `value` - A string slice to check for a match.
    ///
    /// # Returns
    ///
    /// The length of the matched part of the string, or 0 if there is no match.
    pub fn matches(&self, value: &str) -> usize {
        match self {
            RegexVal::Literal(l) => {
                if value.starts_with(*l) {
                    l.len_utf8()
                } else {
                    0
                }
            }
            RegexVal::Wildcard => {
                if let Some(c) = value.chars().next() {
                    c.len_utf8()
                } else {
                    0
                }
            }
            RegexVal::BracketExpression { chars, is_negated } => {
                let next_char = value.chars().next();
                let matches = next_char.map_or(false, |c| chars.contains(&c));
                if matches != *is_negated {
                    next_char.map_or(0, |c| c.len_utf8())
                } else {
                    0
                }
            }
            RegexVal::Class(class) => {
                let next_char = value.chars().next();
                let matches = next_char.map_or(false, |c| class.matches(&c));
                if matches {
                    next_char.map_or(0, |c| c.len_utf8())
                } else {
                    0
                }
            }
        }
    }
}
