use lazy_static::lazy_static;
use std::{
    fmt::Display,
    mem,
    sync::{atomic::AtomicPtr, Mutex},
};

lazy_static! {
    pub static ref FILENAME: Mutex<String> = Mutex::new(String::new());
}

#[derive(Debug, Clone)]
pub struct EvalError {
    filename: String,
    err: String,
}

impl EvalError {
    pub(crate) fn set_filename(filename: &String) {
        FILENAME.lock().unwrap().clear();
        FILENAME.lock().unwrap().push_str(filename);
    }

    pub(crate) fn new(filename: String, err: String) -> Self {
        Self { filename, err }
    }

    pub(crate) fn type_error(msg: String) -> Self {
        Self::new(FILENAME.lock().unwrap().clone(), msg)
    }

    pub(crate) fn divide_zero_error() -> EvalError {
        Self::new(
            FILENAME.lock().unwrap().clone(),
            "divided number cannot be zero".to_string(),
        )
    }
}

impl Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:\nEvalError: {}", self.filename, self.err)
    }
}
