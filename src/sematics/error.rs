use std::fmt::Display;

// define lexical-related errors
#[derive(Debug, Clone)]
pub struct SematicsError {
    err: String,
}

impl SematicsError {
    pub(crate) fn new(err: String) -> Self {
        Self { err }
    }
}

impl Display for SematicsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SematicsError: {}", self.err)
    }
}
