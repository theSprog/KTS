use lazy_static::lazy_static;
use std::{
    fmt::Display,
    mem,
    sync::{atomic::AtomicPtr, Mutex},
};

#[derive(Debug, Clone)]
pub struct EvalError {
    err: String,
}

impl EvalError {
    pub(crate) fn new(err: String) -> Self {
        Self { err }
    }

    pub(crate) fn type_error(msg: String) -> Self {
        Self::new(msg)
    }

    pub(crate) fn divide_zero_error() -> EvalError {
        Self::new("divided number cannot be zero".to_string())
    }
}

impl Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EvalError: {}", self.err)
    }
}
