use crate::error::RegexError;
use crate::evaluated_state::EvaluatedStep;
use crate::regex_rep::RegexRep;
use crate::regex_state::RegexState;
use std::collections::VecDeque;

/// A RegexPart is a regular expression, it is composed of a list of states and a boolean that indicates if the expression ends with a dollar sign.
/// The idea of this struct is to be able to partition a regular expression in different parts.
/// This is useful for regular expressions that contain the OR operator, because we can evaluate each part of the expression separately.
/// If the regular expression does not contain the OR operator, the RegexPart will contain one regular expression, as a RegexPart.
#[derive(Debug)]
pub struct RegexPart {
    pub states: Vec<RegexState>,
    pub ends_with_dollar: bool,
}

impl RegexPart {
    /// Tries to match a single expression with the regular expression part.
    /// It returns a boolean indicating if the expression matches the regular expression part.
    pub fn match_single_expression(self, value: &str) -> Result<bool, RegexError> {
        if !value.is_ascii() {
            return Err(RegexError::NonAsciiInput);
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
                                    index -= size;
                                    continue 'states;
                                }
                                None => return Ok(false),
                            }
                        } else {
                            match_size += s;
                            index += s;
                        }
                    }

                    stack.push(EvaluatedStep {
                        state,
                        match_size,
                        backtrackable: false,
                    })
                }
                RegexRep::Any => {
                    let mut keep_matching = true;
                    while keep_matching {
                        let match_size = state.value.matches(&value[index..]);
                        if match_size != 0 {
                            index += match_size;
                            stack.push(EvaluatedStep {
                                state: state.clone(),
                                match_size,
                                backtrackable: true,
                            });
                        } else {
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
                    stack.push(EvaluatedStep {
                        state,
                        match_size,
                        backtrackable: false,
                    })
                }
            }
        }

        if self.ends_with_dollar {
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

/// This function is used to backtrack when a match is not found.
/// It returns the size of the backtrack.
/// The function receives the current state, a mutable reference to the evaluated steps and a mutable reference to the next states.
/// The function pops the evaluated steps until it finds a backtrackable step, then it returns the size of the backtrack.
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
