use crate::{lexer::{token_kind::{TokenKind, KeyWordKind}, token::Token}, compiler_internal_error, ast::ast_node::literal::Literal};

use super::{Parser, error::ParserError};

impl Parser {
    // expect token error
    pub(super) fn expect_error(&mut self, outline: &str, expects: &str) -> ParserError {
        let cur = self.peek().unwrap();
        self.report_error(&format!(
            "{}: Expect [{}] but got token [ {} ] ({})",
            outline,
            expects,
            cur.peek_value(),
            cur.peek_kind()
        ))
    }

    pub(super) fn unsupported_error(&mut self, unsupported: &str) -> ParserError {
        self.report_error(&format!("Sorry, but now {} is not supported", unsupported))
    }

    pub(super) fn eat(&mut self, kind: TokenKind) -> Result<(), ParserError> {
        if let Some(token) = self.peek() {
            match token.peek_kind() == kind {
                true => {
                    self.index += 1;
                    Ok(())
                }
                false => {
                    if kind == TokenKind::SemiColon && self.is_new_line() {
                        Err(self.report_error(&format!(
                            "you might forgot ';' in the end of line[{}]",
                            self.tokens.get(self.index - 1).unwrap().peek_line()
                        )))
                    }else {
                        Err(self.expect_error("Token Dismatch", &kind.to_string()))
                    }

                }
            }
        } else {
            compiler_internal_error!("Can not eat token because there is no token");
        }
    }

    pub(super) fn is_eos(&mut self) -> bool {
        match self.peek_kind() {
        // 用分号可以  xxx; yyy
            TokenKind::SemiColon 
        // 收尾可以
            | TokenKind::RightBracket
        // 结尾也可以
            | TokenKind::EOF => true,
        // 换行也可以
            _ => self.is_new_line()
        }
    }

    pub(super) fn eat_eos(&mut self) -> Result<(), ParserError> {
        if self.is_eos() {
            if self.peek_kind() == TokenKind::SemiColon {
                self.eat(TokenKind::SemiColon)?;
                return Ok(());
            }
            return Ok(());
        } else {
            Err(self.expect_error("EOS", "; or close-brace or newline"))
        }
    }

    pub(super) fn is_literal(&self) -> bool {
        self.tokens
            .get(self.index)
            .map_or(false, |token| match token.peek_kind() {
                TokenKind::String
                | TokenKind::Number
                | TokenKind::KeyWord(KeyWordKind::True)
                | TokenKind::KeyWord(KeyWordKind::False)
                | TokenKind::KeyWord(KeyWordKind::Null) => true,

                _ => false,
            })
    }

    // 注意，该函数在 extract 的同时也会 eat Token
    pub(super) fn extact_identifier(&mut self) -> Result<String, ParserError> {
        let ident = match self.peek_kind() {
            TokenKind::Identifier => {
                self.eat(TokenKind::Identifier)?;
                self.tokens
                    .get(self.index - 1)
                    .unwrap()
                    .peek_value()
                    .as_str()
            }
            _ => return Err(self.expect_error("Identifier", "identifier")),
        };

        Ok(String::from(ident))
    }

    // 注意，该函数在 extract 的同时也会 eat Token
    pub(super) fn extact_literal(&mut self) -> Result<Literal, ParserError> {
        let literal = match self.peek_kind() {
            TokenKind::String => Literal::String(self.peek().unwrap().peek_value().clone()),

            TokenKind::Number => {
                let string_value = self.peek().unwrap().peek_value().clone();
                // 先处理其余进制
                if string_value.starts_with("0") {
                    Literal::Integer(match string_value.as_bytes() {
                        // 只是单个 0
                        [b'0'] => 0i32,

                        // 特殊的八进制
                        [b'0', b'0'..=b'7', _rest @ ..] => {
                            i32::from_str_radix(&string_value[1..], 8).unwrap()
                        }
                        // 一般情况
                        [f, s, rest @ ..] => {
                            let rest = std::str::from_utf8(rest).unwrap();
                            match (f, s) {
                                (b'0', b'b' | b'B') => i32::from_str_radix(rest, 2).unwrap(),
                                (b'0', b'o' | b'O') => i32::from_str_radix(rest, 8).unwrap(),
                                (b'0', b'x' | b'X') => i32::from_str_radix(rest, 16).unwrap(),
                                _ => return Err(self.report_error("Unrecognized number")),
                            }
                        }

                        _ => compiler_internal_error!("Why it can be here?"),
                    })
                } else {
                    // 再处理最常见的两种情况
                    if let Ok(integer) = string_value.parse::<i32>() {
                        Literal::Integer(integer)
                    } else if let Ok(float) = string_value.parse::<f64>() {
                        Literal::Number(float)
                    } else {
                        compiler_internal_error!("Why it can be here?")
                    }
                }
            }

            TokenKind::KeyWord(KeyWordKind::True) => Literal::Boolean(true),
            TokenKind::KeyWord(KeyWordKind::False) => Literal::Boolean(false),
            TokenKind::KeyWord(KeyWordKind::Null) => Literal::Null,

            _ => return Err(self.expect_error("Literal", "literal")),
        };

        self.index += 1;
        Ok(literal)
    }

    pub(super) fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    pub(super) fn peek_kind(&self) -> TokenKind {
        match self.peek() {
            Some(token) => token.peek_kind(),
            None => TokenKind::EOF,
        }
    }

    pub(super) fn look_ahead(&self) -> TokenKind {
        match self.tokens.get(self.index + 1) {
            Some(token) => token.peek_kind(),
            None =>  TokenKind::EOF,
        }
    }

    pub(super) fn look_ahead2(&self) -> Option<TokenKind> {
        match self.tokens.get(self.index + 2) {
            Some(token) => Some(token.peek_kind()),
            None => None,
        }
    }

    pub(super) fn is_new_line(&self) -> bool {
        if let (Some(current), Some(pre)) =
            (self.tokens.get(self.index), self.tokens.get(self.index - 1))
        {
            current.peek_line() > pre.peek_line()
        } else {
            false
        }
    }

    pub(super) fn kind_is(&self, kind: TokenKind) -> bool {
        match self.peek() {
            Some(token) => token.kind_is(kind),
            None => false,
        }
    }

    pub(super) fn pre_peek_kind(&self) -> TokenKind {
        let pre_token = self.tokens.get(self.index - 1);
        match pre_token {
            Some(pre_token) => pre_token.peek_kind(),
            None => TokenKind::EOF,
        }
    }

    pub(super) fn prekind_is(&self, kind: TokenKind) -> bool {
        let pre_token = &self.tokens[self.index - 1];
        return pre_token.kind_is(kind);
    }
}