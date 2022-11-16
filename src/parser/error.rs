use std::fmt::Display;

// define lexical-related errors
#[derive(Debug, Clone)]
pub struct ParserError(String);
impl ParserError {
    pub(crate) fn new(s: String) -> Self {
        ParserError(s)
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
