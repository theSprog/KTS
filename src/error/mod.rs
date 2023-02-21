use crate::compiler::Compiler;
use crate::eval::error::EvalError;
use crate::parser::error::ParserError;
use crate::{lexer::error::LexerError, sematics::error::SematicsError};
use colored::Colorize;

use std::{
    error::Error,
    fmt::{Debug, Display},
    process,
};

pub fn err_exit<E: Error>(err: E) -> ! {
    eprintln!("Error: {}", format!("{}", err).red());
    process::exit(0);
}

// 定义抽象的解析错误
#[derive(Debug)]
pub enum TSError {
    LexerError(LexerError),
    ParserError(ParserError),
    SematicsError(SematicsError),
    EvalError(EvalError),
}
impl Error for TSError {}

// 分派抽象错误给具体错误处理逻辑，为了简便，我们只是简单打印
impl Display for TSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TSError::LexerError(e) => write!(f, "{}:\n{}", Compiler::filename(), e),
            TSError::ParserError(e) => write!(f, "{}:\n{}", Compiler::filename(), e),
            TSError::SematicsError(e) => write!(f, "{}:\n{}", Compiler::filename(), e),
            TSError::EvalError(e) => write!(f, "{}:\n{}", Compiler::filename(), e),
        }
    }
}

// 定义转换
impl From<LexerError> for TSError {
    fn from(s: LexerError) -> Self {
        TSError::LexerError(s)
    }
}

impl From<ParserError> for TSError {
    fn from(s: ParserError) -> Self {
        TSError::ParserError(s)
    }
}

impl From<SematicsError> for TSError {
    fn from(s: SematicsError) -> Self {
        TSError::SematicsError(s)
    }
}

impl From<EvalError> for TSError {
    fn from(s: EvalError) -> Self {
        TSError::EvalError(s)
    }
}
