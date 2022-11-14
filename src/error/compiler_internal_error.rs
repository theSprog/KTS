use colored::Colorize;
use std::error::Error;

#[macro_export]
macro_rules! compiler_internal_error {
    ($s:expr) => {{
        crate::error::err_exit(
            crate::error::compiler_internal_error::CompilerInternalError::new(format!(
                "CompilerInternalError: {}",
                $s
            )),
        )
    }};
}

#[derive(Debug)]
pub struct CompilerInternalError(String);

impl CompilerInternalError {
    pub(crate) fn new(s: String) -> Self {
        CompilerInternalError(s)
    }
}

impl Error for CompilerInternalError {}

impl std::fmt::Display for CompilerInternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
