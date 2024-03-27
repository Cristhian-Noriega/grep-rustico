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

    pub fn match_expression(self, value: &str) -> Result<bool, &str> {
        if !value.is_ascii() {
            return Err("el input no es ascii");
        }

        let mut queue = VecDeque::from(self.states);
        let mut index = 0;

        while let Some(state) = queue.pop_front() {
            match state.repetition {
                RegexRep::Exact(n) => {
                    for _ in 0..n {
                        //a matches le paso el input que quiero validar
                        //obtengo el size, cuanto avanzo
                        let size = state.value.matches(&value[index..]);

                        if size == 0 {
                            //todo: check if we can backtrack
                            return Ok(false);
                        }
                        //me muevo todo hacia adelante lo que mathcee
                        index += size;
                    }
                },
                RegexRep::Any => {
                    let mut keep_matching = true;
                    while keep_matching {
                        //mientras siga matcheando en el caso any
                        //matcheo el valor, si me devuelve algo me voy adelante
                        //y vuelvo a matchear
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