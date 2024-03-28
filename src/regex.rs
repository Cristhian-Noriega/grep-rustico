use std::collections::VecDeque;
use crate::regex_state::RegexState;
use crate::regex_value::RegexVal;
use crate::regex_rep::RegexRep;
use crate::evaluated_state::EvaluatedStep;

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


    pub fn match_expression(self, value: &str) -> Result<bool, &str> {
        if !value.is_ascii() {
            return Err("el input no es ascii");
        }

        let mut queue = VecDeque::from(self.states);
        let mut stack = Vec::new();
        let mut index = 0;

        'states: while let Some(state) = queue.pop_front() {
            match state.repetition {
                RegexRep::Exact(n) => {
                    let mut match_size = 0;
                    for _ in 0..n {
                        let s = state.value.matches(&value[index..]);
                        
                        if s == 0 {
                            match backtrack(state, &mut stack, &mut queue) {
                                Some(size) => {
                                    println!("entre aca y reste a index");
                                    println!("index es: {:?}", index);
                                    println!("size es: {:?}", size);
                                    index -= 1;
                                    println!("index ahora es: {:?}", index);
                                    continue 'states;
                                }
                                None => return Ok(false),
                            }
                            
                        } else {
                            match_size += s;
                            index += s;
                        }
                    }
                    stack.push(EvaluatedStep{
                        state: state,
                        match_size: index,
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
                                match_size: index,
                                backtrackable: true,
                            });
                        } else {
                            keep_matching = false;
                        }
                    }
                } 
                // RegexRep::Range { min, max } => todo!(),      
                _ => return Ok(false),        
            }
        }
        Ok(true)
    }


    pub fn match_pattern(mut self, value: &str) -> Result<bool, &str> {
        if self.states.is_empty() {
            return Ok(false);
        }

        if let Some(first_state) = self.states.first() {
            if let RegexVal::Literal('^') = first_state.value {
                // First element is '^', call match_expression directly
                return self.match_expression(value);
            }
        }

        // Modify the regex struct to add '.' and '*' at the first two positions
        self.states.insert(0, RegexState {
            value: RegexVal::Wildcard,
            repetition: RegexRep::Exact(1),
        });
        self.states.insert(1, RegexState {
            value: RegexVal::Wildcard,
            repetition: RegexRep::Any,
        });

        // Call match_expression with the modified regex
        self.match_expression(value)
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
        if e.backtrackable {
            println!("Backtracking {:?}", backtrack_size);
            return Some(backtrack_size);
        } else {
            next.push_front(e.state);
        }
    }
    None

}




