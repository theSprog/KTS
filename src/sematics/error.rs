use std::fmt::Display;

// define lexical-related errors
#[derive(Debug, Clone)]
pub struct SematicsError(String);
impl SematicsError {
    pub(crate) fn new(s: String) -> Self {
        SematicsError(s)
    }
}

impl Display for SematicsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
