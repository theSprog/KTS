use self::compiler_internal_error::CompilerInternalError;
use crate::lexer::error::LexerError;
use crate::parser::error::ParserError;

use std::{
    error::Error,
    fmt::{Debug, Display}, process,
};

pub mod compiler_internal_error;

pub fn err_exit<E: Error>(err: E) -> ! {
    eprintln!("{}", err);
    process::exit(1);
}

// 定义抽象的解析错误
#[derive(Debug)]
pub enum TSError {
    CompilerInternalError(CompilerInternalError),
    LexerError(LexerError),
    ParserError(ParserError),
}
impl Error for TSError {}

// 分派抽象错误给具体错误处理逻辑，为了简便，我们只是简单打印
impl Display for TSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TSError::LexerError(e) => Display::fmt(&e, f),
            TSError::ParserError(e) => Display::fmt(&e, f),
            TSError::CompilerInternalError(e) => Display::fmt(&e, f),
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

impl From<CompilerInternalError> for TSError {
    fn from(s: CompilerInternalError) -> Self {
        TSError::CompilerInternalError(s)
    }
}
