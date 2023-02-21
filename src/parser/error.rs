use std::fmt::Display;

// define lexical-related errors
#[derive(Debug, Clone)]
pub struct ParserError {
    err: String,
}

impl ParserError {
    pub(crate) fn new(err: String) -> Self {
        Self { err }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParserError: {}", self.err)
    }
}
