#[derive(Debug)]
pub enum RegexRep {
    Any,
    Exact(usize), 
    // Range{
    //     min: Option<usize>,
    //     max: Option<usize>
    // }
}

impl RegexRep {
    pub fn is_exact(&self) -> bool {
        match self {
            RegexRep::Exact(_) => true,
            _ => false,
        }
    }
}