#[derive(Debug)]
pub struct MatchResult {
    pub start: usize,
    pub end: usize,
    pub matched: String 
}

impl MatchResult {
    pub fn new(start: usize, end: usize, matched: String) -> MatchResult {
        MatchResult {
            start,
            end,
            matched
        }
    }

    pub fn range(&self) -> (usize, usize) {
        (self.start, self.end)
    }
}

