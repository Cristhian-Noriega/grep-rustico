use crate::regex_part::RegexPart;
use crate::regex_state::RegexState;
use crate::regex_value::RegexVal;
use crate::regex_rep::RegexRep;
//use crate::bracket_expression::BracketExpression;

#[derive(Debug)]
pub struct Regex {
    parts: Vec<RegexPart>
}

impl Regex { 
    //parseo de una regex
    pub fn new(expression: &str) -> Result<Self, &str> {
        //let mut states: Vec<RegexState> = vec![];
        let mut ends_with_dollar = false;
        let mut parts: Vec<RegexPart> = vec![];

        //si no tiene ^ al principio le agrego un wildcard
        let expressions: Vec<&str> = expression.split('|').collect();

        
        for expr in expressions {
            let mut states: Vec<RegexState> = vec![];

            if !expression.contains('^') {
                states.push(RegexState {
                    value: RegexVal::Wildcard,
                    repetition: RegexRep::Any,
                });
            }

            let mut chars_iter = expr.chars();
            while let Some(c) = chars_iter.next(){
                let state: Option<RegexState> = match c {
                    //Caso wildcard . (matchea cualquier char una vez)
                    '.' => Some(RegexState{ 
                        value: RegexVal::Wildcard,
                        repetition: RegexRep::Exact(1)
                    }),
                    //Caso literal
                    'a'..='z' => Some(RegexState{ 
                        value: RegexVal::Literal(c),
                        repetition:RegexRep::Exact(1) 
                    }),

                    //Caso *  (el char anterior lo puedo mathear cualquier cant de veces)
                    '*' => {
                        //el ultimo char le cambio el field repetition
                        if let Some(last) = states.last_mut() {
                            last.repetition = RegexRep::Any;
                        } else {
                            return Err("se encontro un caracter '*' inesperado")
                        }

                        None
                            
                    },

                    //Caso \ paso al siguiente caracter y lo tomo como literal
                    '\\' => match chars_iter.next(){
                        Some(literal) => Some(RegexState{ 
                            value: RegexVal::Literal(literal),
                            repetition:RegexRep::Exact(1) 
                        }),

                        None => return Err("se encontro un error"),
                    } 

                    '?' => {
                        //el ultimo char le cambio el field repetition
                        if let Some(last) = states.last_mut() {
                            last.repetition = RegexRep::Range {
                                min: Some(0),
                                max: Some(1),
                            };
                        } else {
                            return Err("se encontro un caracter '?' inesperado")
                        }
                        None
                    },

                    '+' => {
                        //el ultimo char le cambio el field repetition
                        if let Some(last) = states.last_mut() {
                            last.repetition = RegexRep::Range {
                                min: Some(1),
                                max: None,
                            };
                        } else {
                            return Err("se encontro un caracter '+' inesperado")
                        }
                        None
                    },

                    //Caso ^ dejo a match expresion que funcione normalmente
                    '^' => {
                        if expression.starts_with('^') {
                            None
                        } else {
                            return Err("'^' is not at the beginning of the expression");
                        }

                    }
                    //caso $ ends_with_dollar
                    '$' => {
                        if chars_iter.next().is_none() {
                            ends_with_dollar = true;
                            break;
                        } else {
                            return Err("'$' is not at the end of the expression");
                        }
                    }
                    //caso Or |
                    '|' =>  None,
                    //caso Brackets []
                    '[' => {
                        let mut bracket_expression = Vec::new();
                        let mut is_negated = false;
                        while let Some(c) = chars_iter.next() {
                            if c == '^' {
                                is_negated = true;
                                continue;
                            }
                            if c == ']' {
                                break;
                            }
                            bracket_expression.push(c);
                        }
                        if bracket_expression.is_empty() {
                            return Err("Empty bracket expression");
                        }
                        Some( RegexState {
                                value: RegexVal::BracketExpression{
                                    chars: bracket_expression,
                                    is_negated,
                                },
                                repetition: RegexRep::Exact(1),
                            })
                    }
                    _ => return Err("Hubo un eror")
                };
                if let Some(s) = state {
                        states.push(s);
                    }
                ends_with_dollar = false;   
            }
            parts.push(RegexPart{states, ends_with_dollar});
        }
        Ok(Regex{parts})
    }


    pub fn match_expression(self, value: &str) -> Result<bool, &str> {
        if !value.is_ascii() {
            return Err("el input no es ascii");
        }

        for part in self.parts {
            if part.match_sigle_expression(value)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_expression_question_mark() {
        assert_eq!(Regex::new("a?b").unwrap().match_expression("b"), Ok(true));
        assert_eq!(Regex::new("a?b").unwrap().match_expression("ab"), Ok(true));
        assert_eq!(Regex::new("a?b").unwrap().match_expression("aab"), Ok(true));
        assert_eq!(Regex::new("a?b").unwrap().match_expression("aa"), Ok(false));
    }

    #[test]
    fn test_match_expression_wildcard() {
        assert_eq!(Regex::new("a.b").unwrap().match_expression("aab"), Ok(true));
        assert_eq!(Regex::new("a.b").unwrap().match_expression("axb"), Ok(true));
        assert_eq!(Regex::new("a.b").unwrap().match_expression("abc"), Ok(false));
    }

    #[test]
    fn test_match_expression_wildcard_any() {
        assert_eq!(Regex::new("a.*").unwrap().match_expression("abc"), Ok(true));
        assert_eq!(Regex::new("a.*").unwrap().match_expression("a"), Ok(true));
        assert_eq!(Regex::new("ab.*c").unwrap().match_expression("abzzzc"), Ok(true));
        assert_eq!(Regex::new("ab.*cd").unwrap().match_expression("abzzzcd"), Ok(true));
    }

    #[test]
    fn test_match_expression_literals() {
        assert_eq!(Regex::new("abc").unwrap().match_expression("abc"), Ok(true));
        assert_eq!(Regex::new("abc").unwrap().match_expression("ab"), Ok(false));
        assert_eq!(Regex::new("abc").unwrap().match_expression("abcd"), Ok(true));
        assert_eq!(Regex::new("bc").unwrap().match_expression("abcd"), Ok(true));
    }

    #[test]
    fn test_match_expression_star() {
        assert_eq!(Regex::new("a*b").unwrap().match_expression("b"), Ok(true));
        assert_eq!(Regex::new("a*b").unwrap().match_expression("ab"), Ok(true));
        assert_eq!(Regex::new("a*b").unwrap().match_expression("aaab"), Ok(true));
        assert_eq!(Regex::new("a*b").unwrap().match_expression("aa"), Ok(false));
    }

    #[test]
    fn test_match_expression_plus() {
        assert_eq!(Regex::new("a+b").unwrap().match_expression("ab"), Ok(true));
        assert_eq!(Regex::new("a+b").unwrap().match_expression("aab"), Ok(true));
        assert_eq!(Regex::new("a+b").unwrap().match_expression("aa"), Ok(false));
        assert_eq!(Regex::new("go+gle").unwrap().match_expression("gogle"), Ok(true));
        assert_eq!(Regex::new("go+gle").unwrap().match_expression("gooogle"), Ok(true));
        assert_eq!(Regex::new("go+gle").unwrap().match_expression("ggle"), Ok(false));
    }

    #[test]
    fn test_match_expression_caret() {
        assert_eq!(Regex::new("^a").unwrap().match_expression("a"), Ok(true));
        assert_eq!(Regex::new("^a").unwrap().match_expression("ba"), Ok(false));
        assert_eq!(Regex::new("^a").unwrap().match_expression("ab"), Ok(true));
        assert_eq!(Regex::new("^ab").unwrap().match_expression("ba"), Ok(false));
    }

    #[test]
    fn test_match_expression_end_of_line() {
        assert_eq!(Regex::new("a$").unwrap().match_expression("a"), Ok(true));
        assert_eq!(Regex::new("a$").unwrap().match_expression("ba"), Ok(true));
        assert_eq!(Regex::new("a$").unwrap().match_expression("wwwa"), Ok(true));
        assert_eq!(Regex::new("ab$").unwrap().match_expression("ba"), Ok(false));
        assert_eq!(Regex::new("ab$").unwrap().match_expression("ab"), Ok(true));
        assert_eq!(Regex::new("a$").unwrap().match_expression("abb"), Ok(false));
        assert_eq!(Regex::new("og$").unwrap().match_expression("dog"), Ok(true));
    }

    #[test]
    fn test_match_expression_or() {
        assert_eq!(Regex::new("a|b").unwrap().match_expression("a"), Ok(true));
        assert_eq!(Regex::new("a|b").unwrap().match_expression("b"), Ok(true));
        assert_eq!(Regex::new("a|b").unwrap().match_expression("c"), Ok(false));
        assert_eq!(Regex::new("a|b").unwrap().match_expression("ab"), Ok(true));
        assert_eq!(Regex::new("a|b").unwrap().match_expression("ba"), Ok(true));
        assert_eq!(Regex::new("cat|dog").unwrap().match_expression("dog"), Ok(true));
    }

    #[test]
    fn test_match_expression_brackets() {
        assert_eq!(Regex::new("[a]").unwrap().match_expression("a"), Ok(true));
        assert_eq!(Regex::new("[a]").unwrap().match_expression("ab"), Ok(true));
        assert_eq!(Regex::new("[abc]").unwrap().match_expression("c"), Ok(true));
        assert_eq!(Regex::new("[abc]").unwrap().match_expression("ab"), Ok(true));
        assert_eq!(Regex::new("v[aeiou]c[aeiou]l").unwrap().match_expression("vocal"), Ok(true));
        assert_eq!(Regex::new("v[aeiou]c[aeiou]l").unwrap().match_expression("voucal"), Ok(false));
        assert_eq!(Regex::new("v[aeo]c[iou]l").unwrap().match_expression("vocal"), Ok(false));
    }   

    #[test]
    fn test_match_expression_negated_brackets() {
        assert_eq!(Regex::new("mo[^aeiou]tadela").unwrap().match_expression("mortadela"), Ok(true));
        assert_eq!(Regex::new("mo[^aeiou]tadela").unwrap().match_expression("mootadela"), Ok(false));
        assert_eq!(Regex::new("a[^abc]bc").unwrap().match_expression("abc"), Ok(false));
        assert_eq!(Regex::new("a[^abc]bc").unwrap().match_expression("aebc"), Ok(true));
        assert_eq!(Regex::new("a[^abc]bc").unwrap().match_expression("aabc"), Ok(false));
    }
}


