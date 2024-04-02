use crate::regex_class::RegexClass;

#[derive(Debug, Clone)]
pub enum RegexVal {
    Literal(char),
    Wildcard,
    BracketExpression{
        chars: Vec<char>, 
        is_negated: bool
    },
    Class(RegexClass),
}

impl RegexVal {
    pub fn matches(&self, value: &str) -> usize {
        match self {
            RegexVal::Literal(l) => {
                if value.chars().next() == Some(*l) {
                    l.len_utf8()
                } else {
                    0
                }
            },
            RegexVal::Wildcard => {
                if let Some(c) = value.chars().next() {
                    c.len_utf8()
                } else {
                    0
                }
            },
            RegexVal::BracketExpression { chars, is_negated } => {
                let next_char = value.chars().next();
                let matches = next_char.map_or(false, |c| chars.contains(&c));
                if matches == *is_negated {
                    return 0;
                } else {
                    return next_char.map_or(0, |c| c.len_utf8());
                }
            },
            RegexVal::Class(class) => {
                let next_char = value.chars().next();
                let matches = next_char.map_or(false, |c| class.matches(&c));
                if matches {
                    return next_char.map_or(0, |c| c.len_utf8());
                } else {
                    
                    return 0;
                }
            }
        }
    }
}



