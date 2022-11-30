// define lexical-related errors
#[derive(Debug)]
pub struct LexerError {
    filename: String,
    err: String,
}

impl LexerError {
    pub(crate) fn new(filename: String, err: String) -> Self {
        Self { filename, err }
    }
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:\nLexerError: {}", self.filename, self.err)
    }
}
