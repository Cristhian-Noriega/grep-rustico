
#[derive(Debug,Clone)]
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
    pub fn from_str(class_name: &str) -> Option<Self> {
        match class_name {
            ":alpha:" => Some(RegexClass::Alpha),
            ":digit:" => Some(RegexClass::Digit),
            ":alnum:" => Some(RegexClass::Alnum),
            ":lower:" => Some(RegexClass::Lower),
            ":upper:" => Some(RegexClass::Upper),
            ":space:" => Some(RegexClass::Space),
            ":punct:" => Some(RegexClass::Punct),
            _ => None,
        }
    }

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
