// define lexical-related errors
#[derive(Debug)]
pub struct LexerError {
    err: String,
}

impl LexerError {
    pub(crate) fn new(err: String) -> Self {
        Self { err }
    }
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LexerError: {}", self.err)
    }
}
