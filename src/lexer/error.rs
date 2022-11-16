// define lexical-related errors
#[derive(Debug)]
pub struct LexerError(String);
impl LexerError {
    pub(crate) fn new(s: String) -> Self {
        LexerError(s)
    }
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
