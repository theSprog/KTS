use std::fmt::Display;

// define lexical-related errors
#[derive(Debug, Clone)]
pub struct SematicsError {
    filename: String,
    err: String,
}

impl SematicsError {
    pub(crate) fn new(filename: &str, err: String) -> Self {
        Self {
            filename: filename.to_owned(),
            err,
        }
    }
}

impl Display for SematicsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:\nSematicsError: {}", self.filename, self.err)
    }
}
