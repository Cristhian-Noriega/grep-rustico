/// The RegexRep enum represents the repetition of a regex pattern.
/// The repetition can be an exact number of times, a range of times, or any number of times.
/// The repitition can be an unspecified number of times, an exact number of times or in a range of min and max.
/// In the last two cases, the values are stored.
#[derive(Debug, Copy, Clone)]
pub enum RegexRep {
    Any,
    Exact(usize),
    Range {
        min: Option<usize>,
        max: Option<usize>,
    },
}
