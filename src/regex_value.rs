#[derive(Debug, Clone)]
pub enum RegexVal {
    Literal(char),
    Wildcard,
    BracketExpression(Vec<char>)
    //Class(RegexClass),
}

impl RegexVal {
    pub fn matches(&self, value: &str) -> usize {
        match self {
            RegexVal::Literal(l) => {
                if value.chars().next() == Some(*l) {
                    println!("matcheo {}", l.len_utf8());
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
            RegexVal::BracketExpression(chars) => {
                if let Some(c) = value.chars().next() {
                    if chars.contains(&c) {
                        c.len_utf8()
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
        }
    }
}




