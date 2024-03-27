#[derive(Debug)]
pub enum RegexVal {
    Literal(char),
    Wildcard,
    //Class(RegexClass),
}

impl RegexVal {
    //este metodo me dice cuanto debo avanzar en un matcheo sobre el input
    pub fn matches(&self, value:&str) -> usize {
        match self {
            RegexVal::Literal(l) =>{
                //si el sig char matchea un some que adentro tiene un char que busco
                if value.chars().next() == Some(*l) {
                    println!("matcheo {}", l.len_utf8());
                    l.len_utf8() //cant consumida en el input
                } else {
                    0
                }
            },
            RegexVal::Wildcard => {
                //me quedo con el char porque necesito saber el largo
                if let Some(c) = value.chars().next() {
                    c.len_utf8()
                } else {
                    0
                }
            },
            //RegexVal::Class(_) => todo!(),
        }
    } 

}