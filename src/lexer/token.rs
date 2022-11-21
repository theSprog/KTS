use colored::Colorize;
use std::fmt::{Display, Formatter};

use super::token_kind::TokenKind;

#[derive(Debug)]
pub struct Token {
    value: String,
    line: usize,
    kind: TokenKind,
}

impl Token {
    pub fn new(value: &str, line: usize, kind: TokenKind) -> Token {
        Token {
            line,
            kind,
            value: String::from(value),
        }
    }

    pub(crate) fn kind_is(&self, kind: TokenKind) -> bool {
        self.kind == kind
    }

    pub(crate) fn peek_kind(&self) -> TokenKind {
        self.kind
    }

    pub(crate) fn peek_value(&self) -> &str {
        &self.value
    }

    pub(crate) fn peek_line(&self) -> usize {
        self.line
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:>5}][{}] {}",
            self.line,
            self.kind,
            self.value.yellow()
        )
    }
}
