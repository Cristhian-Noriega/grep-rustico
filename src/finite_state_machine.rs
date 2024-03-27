#[derive(Debug, PartialEq, Eq)]
enum State {
    Start,
    Match(char),
    End,
    Error,
}


