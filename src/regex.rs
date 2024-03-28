use std::collections::VecDeque;
use crate::regex_state::RegexState;
use crate::regex_value::RegexVal;
use crate::regex_rep::RegexRep;

#[derive(Debug)]
pub struct Regex {
    states: Vec<RegexState>
}

impl Regex { 
    //parseo de una regex
    pub fn new(expression: &str) -> Result<Self, &str> {
        let mut states: Vec<RegexState> = vec![];
        
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

                _ => return Err("Hubo un eror")
            };

            //con los states que voy coleccionando los agrego al vector de states
            //solo  si es un Some, ya que puede que no lo quiera agregar como *
            if let Some(s) = state {
                states.push(s);
            }

        }
        Ok(Regex{states})
    }

    pub fn match_expression(&self, value: &str) -> Result<bool, &str> {
        let mut input_idx = 0;
        let mut regex_idx = 0;

        while regex_idx < self.states.len() {
            let state = &self.states[regex_idx];
            let mut consumed = 0;

            loop {
                if input_idx >= value.len() {
                    if state.repetition.is_exact() {
                        return Ok(false);
                    } else {
                        break;
                    }
                }

                consumed += state.value.matches(&value[input_idx + consumed..]);

                if consumed == 0 {
                    if state.repetition.is_exact() {
                        return Ok(false);
                    } else {
                        break;
                    }
                }

                input_idx += consumed;
                consumed = 0;

                if !matches!(state.repetition, RegexRep::Any) {
                    break;
                }
            }

            regex_idx += 1;
        }

        Ok(true)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_new() {
        // Test a simple regex
        let regex = Regex::new("abc").unwrap();
        assert_eq!(regex.states.len(), 3);
        assert_eq!(regex.states[0].value, RegexVal::Literal('a'));
        assert_eq!(regex.states[1].value, RegexVal::Literal('b'));
        assert_eq!(regex.states[2].value, RegexVal::Literal('c'));

        // Test a regex with wildcard 
        let regex = Regex::new("a.bc").unwrap();
        assert_eq!(regex.states.len(), 4);
        assert_eq!(regex.states[0].value, RegexVal::Literal('a'));
        assert_eq!(regex.states[1].value, RegexVal::Wildcard);
        assert_eq!(regex.states[2].value, RegexVal::Literal('b'));
        assert_eq!(regex.states[3].value, RegexVal::Literal('c')); 
    }
}
