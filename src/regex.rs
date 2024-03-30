use std::collections::VecDeque;
use crate::regex_state::RegexState;
use crate::regex_value::RegexVal;
use crate::regex_rep::RegexRep;
use crate::evaluated_state::EvaluatedStep;

#[derive(Debug)]
pub struct Regex {
    states: Vec<RegexState>,
    ends_with_dollar: bool,
}

impl Regex { 
    //parseo de una regex
    pub fn new(expression: &str) -> Result<Self, &str> {
        let mut states: Vec<RegexState> = vec![];

        //si no tiene ^ al principio le agrego un wildcard
        
        if !expression.contains('^') {
            states.push(RegexState {
                value: RegexVal::Wildcard,
                repetition: RegexRep::Any,
            });
        }

        let mut ends_with_dollar = false;

        let mut chars_iter = expression.chars();
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
                //TODO  implementar el caso $
                '$' => {
                    if chars_iter.next().is_none() {
                        ends_with_dollar = true;
                        break;
                    } else {
                        return Err("'$' is not at the end of the expression");
                    }
                }
                _ => return Err("Hubo un eror")
            };

            //con los states que voy coleccionando los agrego al vector de states
            //solo  si es un Some, ya que puede que no lo quiera agregar como *
            if let Some(s) = state {
                states.push(s);
            }

        }
        Ok(Regex{states, ends_with_dollar})
    }


    pub fn match_expression(self, value: &str) -> Result<bool, &str> {
        if !value.is_ascii() {
            return Err("el input no es ascii");
        }

        let mut queue = VecDeque::from(self.states);
        let mut stack = Vec::new();
        let mut index = 0;
        let mut ends_with_dollar = self.ends_with_dollar;
       

        'states: while let Some(state) = queue.pop_front() {
            match state.repetition {
                RegexRep::Exact(n) => {
                    let mut match_size = 0;
                    for _ in 0..n {

                        let s = state.value.matches(&value[index..]);


                        println!("El state es {:?} y {:?}", state.value, state.repetition);
                        println!("output de matches {:?}", s);
                        println!("index {:?}", index);
                        // if s == 0 && index == value.len() {
                        //     return Ok(false);
                        // }
                        if s == 0 { //no matcheo
                            match backtrack(state, &mut stack, &mut queue) {
                                Some(size) => {
                                    index -= size;
                                    continue 'states;
                                }
                                None => return Ok(false),
                            }
                            
                        } else { //matcheo
                            println!("ENTRO ACA");
                            match_size += s;
                            index += s;
                        }
                    }

                    

                    stack.push(EvaluatedStep{
                        state: state,
                        match_size,
                        backtrackable: false,
                    })
                },
                RegexRep::Any => {
                    let mut keep_matching = true;
                    while keep_matching {
                        let match_size = state.value.matches(&value[index..]);
                        if match_size != 0 {
                            //println!("entro a any y el match size {:?}", match_size);
                            index += match_size;
                            println!("entre a any y el index es {:?}", index);
                            stack.push(EvaluatedStep{
                                state: state.clone(),
                                match_size,
                                backtrackable: true,
                            });
                        } else {
                            println!("state {:?}", state.value);
                            keep_matching = false;
                        }
                    }
                } 
                RegexRep::Range { min, max } => {
                    let mut match_size = 0;
                    let mut count = 0;
                    loop {
                        let s = state.value.matches(&value[index..]);
                        if s == 0 {
                            break;
                        }
                        match_size += s;
                        index += s;
                        count += 1;
                        if let Some(m) = max {
                            if count == m {
                                break;
                            }
                        }
                    }
                    if let Some(m) = min {
                        if count < m {
                            match backtrack(state, &mut stack, &mut queue) {
                                Some(size) => {
                                    index -= size;
                                    continue 'states;
                                }
                                None => return Ok(false),
                            }
                        }
                    }
                    stack.push(EvaluatedStep{
                        state: state,
                        match_size: match_size,
                        backtrackable: false,
                    })
                }
                _ => return Ok(false),        
            }
        }   

        if ends_with_dollar {
            if index == value.len() {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(true)
        }
    }


}


fn backtrack(
    current: RegexState,
    evaluated: &mut Vec<EvaluatedStep>,
    next: &mut VecDeque<RegexState>,
) -> Option<usize> { 
    let mut backtrack_size = 0;

    next.push_front(current);

    while let Some(e) = evaluated.pop() {
        backtrack_size += e.match_size;
        // println!("evaluated {:?}", e.state.value);
        // println!("es backtrakable?: {:?}", e.backtrackable);
        if e.backtrackable {
            // println!("Backtracking {:?}", backtrack_size);
            return Some(backtrack_size);
        } else {
            next.push_front(e.state);
        }
    }
    None

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
}


