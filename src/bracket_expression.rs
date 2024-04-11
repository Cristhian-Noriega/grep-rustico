/// A bracket expression represents a posible special value in a regex, that matches any single character in the provided list.
/// It has a vector of chars representing the characters that can be matched, and a boolean indicating if the expression is negated.
pub struct BracketExpression {
    pub chars: Vec<char>,
    pub is_negated: bool,
}
