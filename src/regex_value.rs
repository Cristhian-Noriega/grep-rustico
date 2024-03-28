#[derive(Debug)]
pub enum RegexVal {
    Literal(char),
    Wildcard,
    //Class(RegexClass),
}

impl RegexVal {
    pub fn matches(&self, value: &str) -> usize {
        match self {
            RegexVal::Literal(c) => {
                if let Some(idx) = value.find(*c) {
                    return idx + 1;
                }
            }
            RegexVal::Wildcard => {
                if !value.is_empty() {
                    return 1;
                }
            }
        }
        0
    }

}

impl PartialEq for RegexVal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RegexVal::Literal(c1), RegexVal::Literal(c2)) => c1 == c2,
            (RegexVal::Wildcard, RegexVal::Wildcard) => true,
            _ => false,
        }
    }
}