use crate::error::RegexError;
use crate::regex_class::RegexClass;
use crate::regex_part::RegexPart;
use crate::regex_rep::RegexRep;
use crate::regex_state::RegexState;
use crate::regex_value::RegexVal;
use std::vec;

/// Representation of a regular expression, using a vector of `RegexPart` to represent the different parts of the regex.
/// If the regex contains the `|` operator, it will be split into different `RegexPart` objects.
#[derive(Debug)]
pub struct Regex {
    parts: Vec<RegexPart>,
}

impl Regex {
    /// Creates a new `Regex` from a given expression. It contains the main parse logic of a regex.
    ///
    /// # Arguments
    ///
    /// * `expression` - A string slice representing the regular expression.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Regex` instance if the expression is valid, or a RegexError if the expression is invalid.
    pub fn new(expression: &str) -> Result<Self, RegexError> {
        let mut ends_with_dollar = false;
        let mut parts: Vec<RegexPart> = vec![];

        let expressions: Vec<&str> = expression.split('|').collect();

        for expr in expressions {
            let mut states: Vec<RegexState> = vec![];

            if !expression.starts_with('^') {
                states.push(RegexState {
                    value: RegexVal::Wildcard,
                    repetition: RegexRep::Any,
                });
            }

            let mut chars_iter = expr.chars();
            while let Some(c) = chars_iter.next() {
                let state: Option<RegexState> = match c {
                    '.' => Some(RegexState {
                        value: RegexVal::Wildcard,
                        repetition: RegexRep::Exact(1),
                    }),

                    'a'..='z' => Some(RegexState {
                        value: RegexVal::Literal(c),
                        repetition: RegexRep::Exact(1),
                    }),

                    'A'..='Z' => Some(RegexState {
                        value: RegexVal::Literal(c),
                        repetition: RegexRep::Exact(1),
                    }),

                    '0'..='9' => Some(RegexState {
                        value: RegexVal::Literal(c),
                        repetition: RegexRep::Exact(1),
                    }),

                    ' ' => Some(RegexState {
                        value: RegexVal::Literal(' '),
                        repetition: RegexRep::Exact(1),
                    }),

                    '*' => {
                        if let Some(last) = states.last_mut() {
                            last.repetition = RegexRep::Any;
                        } else {
                            return Err(RegexError::InvalidRegularExpression);
                        }

                        None
                    }

                    '\\' => match chars_iter.next() {
                        Some(literal) => Some(RegexState {
                            value: RegexVal::Literal(literal),
                            repetition: RegexRep::Exact(1),
                        }),
                        None => return Err(RegexError::InvalidRegularExpression),
                    },

                    '?' => {
                        if let Some(last) = states.last_mut() {
                            last.repetition = RegexRep::Range {
                                min: Some(0),
                                max: Some(1),
                            };
                        } else {
                            return Err(RegexError::InvalidRegularExpression);
                        }
                        None
                    }

                    '+' => {
                        if let Some(last) = states.last_mut() {
                            last.repetition = RegexRep::Range {
                                min: Some(1),
                                max: None,
                            };
                        } else {
                            return Err(RegexError::InvalidRegularExpression);
                        }
                        None
                    }

                    '^' => {
                        if expression.starts_with('^') {
                            None
                        } else {
                            return Err(RegexError::InvalidRegularExpression);
                        }
                    }

                    '$' => {
                        if chars_iter.next().is_none() {
                            ends_with_dollar = true;
                            break;
                        } else {
                            return Err(RegexError::InvalidRegularExpression);
                        }
                    }

                    '|' => None,

                    '[' => {
                        if let Some(next_char) = chars_iter.next() {
                            match if next_char == '[' {
                                parse_character_class(&mut chars_iter)
                            } else {
                                parse_bracket_expression(&mut chars_iter, next_char)
                            } {
                                Ok(Some(state)) => states.push(state),
                                Ok(None) => (),
                                Err(err) => return Err(err),
                            }
                            continue;
                        }
                        None
                    }

                    '{' => {
                        let repetition = parse_range_repetition(&mut chars_iter)?;
                        if let Some(last) = states.last_mut() {
                            last.repetition = repetition;
                        } else {
                            return Err(RegexError::InvalidRegularExpression);
                        }
                        None
                    }
                    _ => return Err(RegexError::InvalidRegularExpression),
                };
                if let Some(s) = state {
                    states.push(s);
                }
                ends_with_dollar = false;
            }
            parts.push(RegexPart {
                states,
                ends_with_dollar,
            });
        }
        Ok(Regex { parts })
    }

    /// Matches the given value against the regular expression.
    /// It iterates over the different `RegexPart` objects and tries to match the value against each one.
    /// # Arguments
    ///
    /// * `value` - A string slice to match against the regular expression.
    ///
    /// # Returns
    ///
    /// A `Result` containing `true` if the string matches the regular expression, or `false` otherwise.
    pub fn match_expression(self, value: &str) -> Result<bool, RegexError> {
        if !value.is_ascii() {
            return Err(RegexError::NonAsciiInput);
        }

        for part in self.parts {
            match part.match_single_expression(value) {
                Ok(true) => return Ok(true),
                Err(err) => return Err(err),
                _ => (),
            }
        }
        Ok(false)
    }
}

/// Tries to parse a bracket expression in a expression.
/// It receives the next char in the original iteration to decide if is negated or not.
/// # Arguments
///
/// * `chars_iter` - A mutable reference to the character iterator.
/// * `next_char` - The next character after the opening bracket '['.
///
/// # Returns
///
/// An optional `RegexState` representing the parsed bracket expression if it was successful.
fn parse_bracket_expression(
    chars_iter: &mut std::str::Chars<'_>,
    next_char: char,
) -> Result<Option<RegexState>, RegexError> {
    let mut bracket_expression = Vec::new();
    let mut is_negated = false;
    if next_char == '^' {
        is_negated = true;
    } else {
        bracket_expression.push(next_char);
    }
    for c in chars_iter.by_ref() {
        if c == ']' {
            break;
        }
        bracket_expression.push(c);
    }
    if bracket_expression.is_empty() {
        return Ok(None);
    }
    Ok(Some(RegexState {
        value: RegexVal::BracketExpression {
            chars: bracket_expression,
            is_negated,
        },
        repetition: RegexRep::Exact(1),
    }))
}

/// Tries to parse a character class in a expression.
///
/// # Arguments
///
/// * `chars_iter` - A mutable reference to the character iterator.
///
/// # Returns
///
/// An optional `RegexState` representing the parsed character class if it was succesful.
fn parse_character_class(
    chars_iter: &mut std::str::Chars<'_>,
) -> Result<Option<RegexState>, RegexError> {
    let mut character_class = Vec::new();
    for c in chars_iter.by_ref() {
        if c == ']' {
            break;
        }
        character_class.push(c);
    }
    chars_iter.next();
    if character_class.is_empty() {
        return Err(RegexError::InvalidCharacterClassName);
    }
    let class_name = character_class.iter().collect::<String>();

    match RegexClass::from_str_to_class(&class_name) {
        Ok(regex_class) => Ok(Some(RegexState {
            value: RegexVal::Class(regex_class),
            repetition: RegexRep::Exact(1),
        })),
        Err(err) => Err(err),
    }
}
/// Tries to parse a range repetition in a expression.
///
/// # Arguments
///
/// * `chars_iter` - A mutable reference to the character iterator.
///
/// # Returns
///
/// A `Result` containing the parsed `RegexRep` representing the repetition range if it was successful,
fn parse_range_repetition(chars_iter: &mut std::str::Chars<'_>) -> Result<RegexRep, RegexError> {
    let mut min = None;
    let mut max = None;
    let mut parameters = Vec::new();

    for c in chars_iter.by_ref() {
        match c {
            '}' => break,
            ',' => {
                if !parameters.is_empty() {
                    min = parameters.iter().collect::<String>().parse().ok();
                    parameters.clear();
                }
            }
            _ if c.is_ascii_digit() => parameters.push(c),
            _ => return Err(RegexError::InvalidRegularExpression),
        }
    }

    if !parameters.is_empty() {
        max = parameters.iter().collect::<String>().parse().ok();
    }

    Ok(RegexRep::Range { min, max })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_expression_basic() {
        assert_eq!(Regex::new("a").unwrap().match_expression("casa"), Ok(true));
        assert_eq!(
            Regex::new("aa").unwrap().match_expression("haaha"),
            Ok(true)
        );
        assert_eq!(Regex::new("a").unwrap().match_expression("hola"), Ok(true));
    }
    #[test]
    fn test_match_expression_question_mark() {
        assert_eq!(Regex::new("a?b").unwrap().match_expression("b"), Ok(true));
        assert_eq!(Regex::new("a?b").unwrap().match_expression("ab"), Ok(true));
        assert_eq!(Regex::new("a?b").unwrap().match_expression("aab"), Ok(true));
        assert_eq!(Regex::new("a?b").unwrap().match_expression("aa"), Ok(false));
    }

    #[test]
    fn test_match_expression_wildcard() {
        assert_eq!(Regex::new("a.b").unwrap().match_expression("aab"), Ok(true));
        assert_eq!(Regex::new("a.b").unwrap().match_expression("axb"), Ok(true));
        assert_eq!(
            Regex::new("a.b").unwrap().match_expression("abc"),
            Ok(false)
        );
    }

    #[test]
    fn test_match_expression_wildcard_any() {
        assert_eq!(Regex::new("a.*").unwrap().match_expression("abc"), Ok(true));
        assert_eq!(Regex::new("a.*").unwrap().match_expression("a"), Ok(true));
        assert_eq!(
            Regex::new("ab.*c").unwrap().match_expression("abzzzc"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("ab.*cd").unwrap().match_expression("abzzzcd"),
            Ok(true)
        );
    }

    #[test]
    fn test_match_expression_literals() {
        assert_eq!(Regex::new("abc").unwrap().match_expression("abc"), Ok(true));
        assert_eq!(Regex::new("abc").unwrap().match_expression("ab"), Ok(false));
        assert_eq!(
            Regex::new("abc").unwrap().match_expression("abcd"),
            Ok(true)
        );
        assert_eq!(Regex::new("bc").unwrap().match_expression("abcd"), Ok(true));
    }

    #[test]
    fn test_match_expression_star() {
        assert_eq!(Regex::new("a*b").unwrap().match_expression("b"), Ok(true));
        assert_eq!(Regex::new("a*b").unwrap().match_expression("ab"), Ok(true));
        assert_eq!(
            Regex::new("a*b").unwrap().match_expression("aaab"),
            Ok(true)
        );
        assert_eq!(Regex::new("a*b").unwrap().match_expression("aa"), Ok(false));
    }

    #[test]
    fn test_match_expression_plus() {
        assert_eq!(Regex::new("a+b").unwrap().match_expression("ab"), Ok(true));
        assert_eq!(Regex::new("a+b").unwrap().match_expression("aab"), Ok(true));
        assert_eq!(Regex::new("a+b").unwrap().match_expression("aa"), Ok(false));
        assert_eq!(
            Regex::new("go+gle").unwrap().match_expression("gogle"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("go+gle").unwrap().match_expression("gooogle"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("go+gle").unwrap().match_expression("ggle"),
            Ok(false)
        );
    }

    #[test]
    fn test_match_expression_caret() {
        assert_eq!(Regex::new("^a").unwrap().match_expression("a"), Ok(true));
        assert_eq!(Regex::new("^a").unwrap().match_expression("ba"), Ok(false));
        assert_eq!(Regex::new("^a").unwrap().match_expression("ab"), Ok(true));
        assert_eq!(Regex::new("^ab").unwrap().match_expression("ba"), Ok(false));
    }

    #[test]
    fn test_match_expression_end_of_line() {
        assert_eq!(Regex::new("a$").unwrap().match_expression("a"), Ok(true));
        assert_eq!(Regex::new("a$").unwrap().match_expression("ba"), Ok(true));
        assert_eq!(Regex::new("a$").unwrap().match_expression("wwwa"), Ok(true));
        assert_eq!(Regex::new("ab$").unwrap().match_expression("ba"), Ok(false));
        assert_eq!(Regex::new("ab$").unwrap().match_expression("ab"), Ok(true));
        assert_eq!(Regex::new("a$").unwrap().match_expression("abb"), Ok(false));
        assert_eq!(Regex::new("og$").unwrap().match_expression("dog"), Ok(true));
    }

    #[test]
    fn test_match_expression_or() {
        assert_eq!(Regex::new("a|b").unwrap().match_expression("a"), Ok(true));
        assert_eq!(Regex::new("a|b").unwrap().match_expression("b"), Ok(true));
        assert_eq!(Regex::new("a|b").unwrap().match_expression("c"), Ok(false));
        assert_eq!(Regex::new("a|b").unwrap().match_expression("ab"), Ok(true));
        assert_eq!(Regex::new("a|b").unwrap().match_expression("ba"), Ok(true));
        assert_eq!(
            Regex::new("cat|dog").unwrap().match_expression("dog"),
            Ok(true)
        );
    }

    #[test]
    fn test_match_expression_brackets() {
        assert_eq!(Regex::new("[a]").unwrap().match_expression("a"), Ok(true));
        assert_eq!(Regex::new("[a]").unwrap().match_expression("ab"), Ok(true));
        assert_eq!(Regex::new("[abc]").unwrap().match_expression("c"), Ok(true));
        assert_eq!(
            Regex::new("[abc]").unwrap().match_expression("ab"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("v[aeiou]cal").unwrap().match_expression("vocal"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("v[aeiou]c[aeiou]l")
                .unwrap()
                .match_expression("vocal"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("v[aeiou]c[aeiou]l")
                .unwrap()
                .match_expression("voucal"),
            Ok(false)
        );
        assert_eq!(
            Regex::new("v[aeo]c[iou]l")
                .unwrap()
                .match_expression("vocal"),
            Ok(false)
        );
    }

    #[test]
    fn test_match_expression_negated_brackets() {
        assert_eq!(
            Regex::new("mo[^aeiou]tadela")
                .unwrap()
                .match_expression("mortadela"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("mo[^aeiou]tadela")
                .unwrap()
                .match_expression("mootadela"),
            Ok(false)
        );
        assert_eq!(
            Regex::new("a[^abc]bc").unwrap().match_expression("abc"),
            Ok(false)
        );
        assert_eq!(
            Regex::new("a[^abc]bc").unwrap().match_expression("aebc"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("a[^abc]bc").unwrap().match_expression("aabc"),
            Ok(false)
        );
    }

    #[test]
    fn test_match_expression_alternance_precedence() {
        assert_eq!(
            Regex::new("abc|de+f").unwrap().match_expression("abc"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("abc|de+f").unwrap().match_expression("def"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("abc|de+f").unwrap().match_expression("deef"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("abc|de+f").unwrap().match_expression("df"),
            Ok(false)
        );
        assert_eq!(
            Regex::new("abc|de+f").unwrap().match_expression("cde"),
            Ok(false)
        );
    }

    #[test]
    fn test_match_expression_multiples_wildcard_any() {
        assert_eq!(
            Regex::new("ab.*c.*f")
                .unwrap()
                .match_expression("abzzzczzzf"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("ab.*c.*f").unwrap().match_expression("abzzzcf"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("ab.*c.*f").unwrap().match_expression("abcf"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("ab.*c.*f").unwrap().match_expression("abzcfe"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("ab.*c.*f").unwrap().match_expression("abzczzz"),
            Ok(false)
        );
    }

    #[test]
    fn test_match_expression_range() {
        assert_eq!(Regex::new("a{2}").unwrap().match_expression("aa"), Ok(true));
        assert_eq!(
            Regex::new("ab{2,4}cd").unwrap().match_expression("aabbcd"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("ab{2,4}cd").unwrap().match_expression("aabbbcd"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("ab{2,4}cd").unwrap().match_expression("abbbbcd"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("ab{2,4}cd").unwrap().match_expression("abcd"),
            Ok(false)
        );
        assert_eq!(
            Regex::new("ab{2,4}cd")
                .unwrap()
                .match_expression("abbbbbbcd"),
            Ok(false)
        );
        assert_eq!(
            Regex::new("ab{2,4}cd").unwrap().match_expression("cabbcd"),
            Ok(true)
        );
    }

    #[test]
    fn test_match_expression_range_max() {
        assert_eq!(
            Regex::new("a{,3}b").unwrap().match_expression("aab"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("a{,3}b").unwrap().match_expression("ab"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("a{,3}b").unwrap().match_expression("b"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("a{,3}b").unwrap().match_expression("eaaaab"),
            Ok(true)
        );
    }

    #[test]
    fn test_match_expression_range_min() {
        assert_eq!(
            Regex::new("ea{3,}b").unwrap().match_expression("aaab"),
            Ok(false)
        );
        assert_eq!(
            Regex::new("ea{3,}b").unwrap().match_expression("aaab"),
            Ok(false)
        );
        assert_eq!(
            Regex::new("ea{3,}b").unwrap().match_expression("eaaab"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("efga{3,}").unwrap().match_expression("efgaa"),
            Ok(false)
        );
        assert_eq!(
            Regex::new("efga{3,}").unwrap().match_expression("efgaaaa"),
            Ok(true)
        );
    }

    #[test]
    fn test_match_expression_character_classes() {
        assert_eq!(
            Regex::new("[[:alpha:]]").unwrap().match_expression("a"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("[[:alpha:]]").unwrap().match_expression("A"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("[[:alnum:]]hola")
                .unwrap()
                .match_expression("1hola"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("[[:digit:]]").unwrap().match_expression("1"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("[[:digit:]]a").unwrap().match_expression("2b"),
            Ok(false)
        );
        assert_eq!(
            Regex::new("hola[[:space:]]mundo")
                .unwrap()
                .match_expression("hola mundo"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("el caracter [[:alnum:]] no es un simbolo")
                .unwrap()
                .match_expression("el caracter 2 no es un simbolo"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("[[:upper:]]ascal[[:upper:]]ase")
                .unwrap()
                .match_expression("PascalCase"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("hola [[:alpha:]]+")
                .unwrap()
                .match_expression("hola mundo"),
            Ok(true)
        );
        assert_eq!(
            Regex::new("hola [[:alpha:]]+")
                .unwrap()
                .match_expression("hola a"),
            Ok(true)
        );
    }
}
