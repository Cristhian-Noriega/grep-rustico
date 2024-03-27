use std::collections::VecDeque;

 

pub struct RegexState{
    value: RegexVal,
    repetition: RegexRep,
}


pub struct Regex {
    states: Vec<RegexState>
}



#[derive(Debug)]
pub enum RegexClass {
    Alnum,  
    Alpha,  
    Digit,  
    Lower,  
    Upper,  
    Space,  
    Punct,  
}



#[derive()]
pub enum RegexVal {
    Literal(char),
    Wildcard,
    Class(RegexClass),
}

pub enum RegexRep {
    Any,
    Exact(usize),
    Range{
        min: Option<usize>,
        max: Option<usize>
    }
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

    pub fn match(self, value: &str) -> Result<bool, &str> {
        if !value.is_ascii() {
            return Err("el input no es ascii");
        }

        let mut queue = VecDeque::from(self.states);
        let mut index = 0;

        while let Some(state) = queue.pop_front() {
            match state.repetition {
                RegexRep::Exact(n) => {
                    for _ in 0..n {
                        let size = state.value.matches(&value[index..]);

                        if size == 0 {
                            //todo: check if we can backtrack
                            return Ok(false);
                        }
                        index += size;
                    }
                },
                RegexRep::Any => {
                    let mut keep_matching = true;
                    while keep_matching {
                        let match_size = state.value.matches(&value[index..]);
                        if match_size != 0 {
                            index += match_size;
                        } else {
                            keep_matching = false;
                        }
                    }
                } 
                // RegexRep::Range { min, max } => todo!(),                
            }
        }
        Ok(true)
    }

}