use std::fmt::Display;

// define lexical-related errors
#[derive(Debug, Clone)]
pub struct ParserError {
    filename: String,
    err: String,
}

impl ParserError {
    pub(crate) fn new(filename: String, err: String) -> Self {
        Self { filename, err }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:\nParserError: {}", self.filename, self.err)
    }
}
