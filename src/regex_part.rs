use crate::regex_state::RegexState;
use crate::evaluated_state::EvaluatedStep;
use crate::regex_rep::RegexRep;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct RegexPart {
    pub states: Vec<RegexState>,
    pub ends_with_dollar: bool,
}

impl RegexPart {
    pub fn match_sigle_expression(self, value: &str) -> Result<bool, &str> {
        if !value.is_ascii() {
            return Err("el input no es ascii");
        }

        let mut queue = VecDeque::from(self.states);
        let mut stack = Vec::new();
        let mut index = 0;
        let  ends_with_dollar = self.ends_with_dollar;
       
        'states: while let Some(state) = queue.pop_front() {
            match state.repetition {
                RegexRep::Exact(n) => {
                    let mut match_size = 0;
                    for _ in 0..n {

                        let s = state.value.matches(&value[index..]);
                        if s == 0 { //no matcheo
                            match backtrack(state, &mut stack, &mut queue) {
                                Some(size) => {
                                    index -= size;
                                    continue 'states;
                                }
                                None => return Ok(false),
                            }
                            
                        } else { //matcheo
                            //println!("ENTRO ACA");
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
                            //println!("entre a any y el index es {:?}", index);
                            stack.push(EvaluatedStep{
                                state: state.clone(),
                                match_size,
                                backtrackable: true,
                            });
                        } else {
                            //println!("state {:?}", state.value);
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
                            if count >= m {
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
                        match_size,
                        backtrackable: false,
                    })
                }      
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
        if e.backtrackable {
            return Some(backtrack_size);
        } else {
            next.push_front(e.state);
        }
    }
    None
}