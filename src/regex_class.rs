use crate::error::RegexError;

/// Represents predefined character classes for regulars expressions.

#[derive(Debug, Clone)]
pub enum RegexClass {
    Alpha,
    Alnum,
    Digit,
    Lower,
    Upper,
    Space,
    Punct,
}

impl RegexClass {
    /// Converts a string representation of a character class to a `RegexClass` enum variant.
    ///
    /// # Arguments
    ///
    /// * `class_name` - The string representation of the character class.
    ///
    /// # Returns
    ///
    /// Returns a `RegexClass` enum variant if the string is a valid character class name,
    /// otherwise  returns an `InvalidCharacterClassName` error.
    pub fn from_str_to_class(class_name: &str) -> Result<Self, RegexError> {
        match class_name {
            ":alpha:" => Ok(RegexClass::Alpha),
            ":digit:" => Ok(RegexClass::Digit),
            ":alnum:" => Ok(RegexClass::Alnum),
            ":lower:" => Ok(RegexClass::Lower),
            ":upper:" => Ok(RegexClass::Upper),
            ":space:" => Ok(RegexClass::Space),
            ":punct:" => Ok(RegexClass::Punct),
            _ => Err(RegexError::InvalidCharacterClassName),
        }
    }

    /// Checks if a character matches the character class.
    ///
    /// # Arguments
    ///
    /// * `c` - The character to be checked.
    ///
    /// # Returns
    ///
    /// Returns `true` if the character matches the character class, otherwise returns `false`.
    pub fn matches(&self, c: &char) -> bool {
        match self {
            RegexClass::Alpha => c.is_ascii_alphabetic(),
            RegexClass::Alnum => c.is_alphanumeric(),
            RegexClass::Digit => c.is_ascii_digit(),
            RegexClass::Lower => c.is_ascii_lowercase(),
            RegexClass::Upper => c.is_ascii_uppercase(),
            RegexClass::Space => c.is_whitespace(),
            RegexClass::Punct => c.is_ascii_punctuation(),
        }
    }
}
