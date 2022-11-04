use crate::compiler_internal_error;
use crate::lexer::token_kind::{KeyWordKind, TokenKind};
use lazy_static::lazy_static;
use regex::Regex;

use std::{collections::HashMap, str};

use self::{error::LexerError, token::Token};

pub(crate) mod error;
pub mod token;
pub mod token_kind;

lazy_static! {
    pub static ref KEYWORD: HashMap<&'static str, TokenKind> = {
        let mut map = HashMap::new();
        map.insert("true", TokenKind::KeyWord(KeyWordKind::True));
        map.insert("false", TokenKind::KeyWord(KeyWordKind::False));
        map.insert("null", TokenKind::KeyWord(KeyWordKind::Null));

        map.insert("break", TokenKind::KeyWord(KeyWordKind::Break));
        map.insert("do", TokenKind::KeyWord(KeyWordKind::Do));
        map.insert("instanceof", TokenKind::KeyWord(KeyWordKind::Instanceof));
        map.insert("typeof", TokenKind::KeyWord(KeyWordKind::Typeof));
        map.insert("case", TokenKind::KeyWord(KeyWordKind::Case));
        map.insert("else", TokenKind::KeyWord(KeyWordKind::Else));
        map.insert("new", TokenKind::KeyWord(KeyWordKind::New));
        map.insert("var", TokenKind::KeyWord(KeyWordKind::Var));
        map.insert("catch", TokenKind::KeyWord(KeyWordKind::Catch));
        map.insert("finally", TokenKind::KeyWord(KeyWordKind::Finally));
        map.insert("return", TokenKind::KeyWord(KeyWordKind::Return));
        map.insert("void", TokenKind::KeyWord(KeyWordKind::Void));
        map.insert("continue", TokenKind::KeyWord(KeyWordKind::Continue));
        map.insert("for", TokenKind::KeyWord(KeyWordKind::For));
        map.insert("switch", TokenKind::KeyWord(KeyWordKind::Switch));
        map.insert("while", TokenKind::KeyWord(KeyWordKind::While));
        map.insert("debugger", TokenKind::KeyWord(KeyWordKind::Debugger));
        map.insert("function", TokenKind::KeyWord(KeyWordKind::Function_));
        map.insert("this", TokenKind::KeyWord(KeyWordKind::This));
        map.insert("with", TokenKind::KeyWord(KeyWordKind::With));
        map.insert("default", TokenKind::KeyWord(KeyWordKind::Default));
        map.insert("if", TokenKind::KeyWord(KeyWordKind::If));
        map.insert("throw", TokenKind::KeyWord(KeyWordKind::Throw));
        map.insert("delete", TokenKind::KeyWord(KeyWordKind::Delete));
        map.insert("in", TokenKind::KeyWord(KeyWordKind::In));
        map.insert("try", TokenKind::KeyWord(KeyWordKind::Try));
        map.insert("as", TokenKind::KeyWord(KeyWordKind::As));
        map.insert("from", TokenKind::KeyWord(KeyWordKind::From));
        map.insert("readonly", TokenKind::KeyWord(KeyWordKind::ReadOnly));
        map.insert("async", TokenKind::KeyWord(KeyWordKind::Async));
        map.insert("class", TokenKind::KeyWord(KeyWordKind::Class));
        map.insert("enum", TokenKind::KeyWord(KeyWordKind::Enum));
        map.insert("extends", TokenKind::KeyWord(KeyWordKind::Extends));
        map.insert("super", TokenKind::KeyWord(KeyWordKind::Super));
        map.insert("const", TokenKind::KeyWord(KeyWordKind::Const));
        map.insert("export", TokenKind::KeyWord(KeyWordKind::Export));
        map.insert("import", TokenKind::KeyWord(KeyWordKind::Import));

        map.insert("implements", TokenKind::KeyWord(KeyWordKind::Implements));
        map.insert("let", TokenKind::KeyWord(KeyWordKind::Let));
        map.insert("private", TokenKind::KeyWord(KeyWordKind::Private));
        map.insert("public", TokenKind::KeyWord(KeyWordKind::Public));
        map.insert("interface", TokenKind::KeyWord(KeyWordKind::Interface));
        map.insert("package", TokenKind::KeyWord(KeyWordKind::Package));
        map.insert("protected", TokenKind::KeyWord(KeyWordKind::Protected));
        map.insert("static", TokenKind::KeyWord(KeyWordKind::Static));
        map.insert("yield", TokenKind::KeyWord(KeyWordKind::Yield));

        map.insert("any", TokenKind::KeyWord(KeyWordKind::Any));
        map.insert("number", TokenKind::KeyWord(KeyWordKind::Number));
        map.insert("boolean", TokenKind::KeyWord(KeyWordKind::Boolean));
        map.insert("string", TokenKind::KeyWord(KeyWordKind::String));
        map.insert("symbol", TokenKind::KeyWord(KeyWordKind::Symbol));

        map.insert("type", TokenKind::KeyWord(KeyWordKind::TypeAlias));
        map.insert("get", TokenKind::KeyWord(KeyWordKind::Get));
        map.insert("set", TokenKind::KeyWord(KeyWordKind::Set));
        map.insert("constructor", TokenKind::KeyWord(KeyWordKind::Constructor));
        map.insert("namespace", TokenKind::KeyWord(KeyWordKind::Namespace));
        map.insert("require", TokenKind::KeyWord(KeyWordKind::Require));
        map.insert("module", TokenKind::KeyWord(KeyWordKind::Module));
        map.insert("declare", TokenKind::KeyWord(KeyWordKind::Declare));
        map.insert("abstract", TokenKind::KeyWord(KeyWordKind::Abstract));
        map.insert("is", TokenKind::KeyWord(KeyWordKind::Is));

        map
    };
}

pub(crate) struct Lexer<'a> {
    bytes: &'a [u8],
    line: usize,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(chars: &'a str) -> Self {
        Self {
            bytes: chars.as_bytes(),
            line: 1,
        }
    }

    pub(crate) fn get_token_stream(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token();
            match token {
                Ok(token) => {
                    let is_eof = token.kind_is(TokenKind::EOF);
                    tokens.push(token);
                    // once token has moved, we could not to access it again,
                    // so we need to early figure out what the kind it is
                    if is_eof {
                        break;
                    }
                }
                Err(e) => return Err(e.into()),
            }
        }

        Ok(tokens)
    }

    pub(crate) fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_unrelated();

        if self.bytes.is_empty() {
            return Ok(self.make_eof_token(self.line));
        }

        // as we had already test the len of bytes before,
        // so we can unwrap it directly
        match self.peek().unwrap() {
            b'(' => Ok(self.make_token("(", self.line, TokenKind::LeftParen)),
            b')' => Ok(self.make_token(")", self.line, TokenKind::RightParen)),
            b'[' => Ok(self.make_token("[", self.line, TokenKind::LeftBrace)),
            b']' => Ok(self.make_token("]", self.line, TokenKind::RightBrace)),
            b'{' => Ok(self.make_token("{", self.line, TokenKind::LeftBracket)),
            b'}' => Ok(self.make_token("}", self.line, TokenKind::RightBracket)),
            b',' => Ok(self.make_token(",", self.line, TokenKind::Comma)),
            b';' => Ok(self.make_token(";", self.line, TokenKind::SemiColon)),
            b'?' => Ok(self.make_token("?", self.line, TokenKind::QuestionMark)),
            b':' => Ok(self.make_token(":", self.line, TokenKind::Colon)),
            b'~' => Ok(self.make_token("~", self.line, TokenKind::BitNot)),
            b'@' => Ok(self.make_token("@", self.line, TokenKind::At)),

            b'a'..=b'z' | b'A'..=b'Z' | b'_' => Ok(self.make_identifier_token()),
            b'0'..=b'9' | b'.' if self.is_number() => Ok(self.make_number_token()?),
            b'"' | b'\'' => Ok(self.make_string_token()?),

            b'!' => match self.bytes {
                [b'!', b'=', b'=', _res @ ..] => {
                    Ok(self.make_token("!==", self.line, TokenKind::IdentityNotEquals))
                }
                [b'!', b'=', _res @ ..] => {
                    Ok(self.make_token("!=", self.line, TokenKind::NotEquals))
                }
                _ => Ok(self.make_token("!", self.line, TokenKind::Not)),
            },

            b'*' => match self.bytes {
                [b'*', b'=', _res @ ..] => {
                    Ok(self.make_token("*=", self.line, TokenKind::MultiplyAssign))
                }
                _ => Ok(self.make_token("*", self.line, TokenKind::Multiply)),
            },

            b'/' => match self.bytes {
                [b'/', b'=', _res @ ..] => {
                    Ok(self.make_token("/=", self.line, TokenKind::DivideAssign))
                }
                _ => Ok(self.make_token("/", self.line, TokenKind::Divide)),
            },

            b'%' => match self.bytes {
                [b'%', b'=', _res @ ..] => {
                    Ok(self.make_token("%=", self.line, TokenKind::ModulusAssign))
                }
                _ => Ok(self.make_token("%", self.line, TokenKind::Modulus)),
            },

            b'.' => match self.bytes {
                [b'.', b'.', b'.', _res @ ..] => {
                    Ok(self.make_token("...", self.line, TokenKind::Ellipsis))
                }
                _ => Ok(self.make_token(".", self.line, TokenKind::Dot)),
            },

            b'+' => match self.bytes {
                [b'+', b'+', _res @ ..] => {
                    Ok(self.make_token("++", self.line, TokenKind::PlusPlus))
                }
                [b'+', b'=', _res @ ..] => {
                    Ok(self.make_token("+=", self.line, TokenKind::PlusAssign))
                }
                _ => Ok(self.make_token("+", self.line, TokenKind::Plus)),
            },

            b'-' => match self.bytes {
                [b'-', b'-', _res @ ..] => {
                    Ok(self.make_token("--", self.line, TokenKind::MinusMinus))
                }
                [b'-', b'=', _res @ ..] => {
                    Ok(self.make_token("-=", self.line, TokenKind::MinusAssign))
                }
                _ => Ok(self.make_token("-", self.line, TokenKind::Minus)),
            },

            b'>' => match self.bytes {
                [b'>', b'>', b'>', b'=', _res @ ..] => {
                    Ok(self.make_token(">>>=", self.line, TokenKind::RightShiftLogicalAssign))
                }
                [b'>', b'>', b'=', _res @ ..] => {
                    Ok(self.make_token(">>=", self.line, TokenKind::RightShiftArithmeticAssign))
                }
                [b'>', b'>', b'>', _res @ ..] => {
                    Ok(self.make_token(">>>", self.line, TokenKind::RightShiftLogical))
                }
                [b'>', b'>', _res @ ..] => {
                    Ok(self.make_token(">>", self.line, TokenKind::RightShiftArithmetic))
                }
                [b'>', b'=', _res @ ..] => {
                    Ok(self.make_token(">=", self.line, TokenKind::GreaterThanEquals))
                }
                _ => Ok(self.make_token(">", self.line, TokenKind::MoreThan)),
            },

            b'<' => match self.bytes {
                [b'<', b'<', b'=', _res @ ..] => {
                    Ok(self.make_token("<<=", self.line, TokenKind::LeftShiftArithmeticAssign))
                }
                [b'<', b'<', _res @ ..] => {
                    Ok(self.make_token("<<", self.line, TokenKind::LeftShiftArithmetic))
                }
                [b'<', b'=', _res @ ..] => {
                    Ok(self.make_token("<=", self.line, TokenKind::LessThanEquals))
                }
                _ => Ok(self.make_token("<", self.line, TokenKind::LessThan)),
            },

            b'=' => match self.bytes {
                [b'=', b'=', b'=', _res @ ..] => {
                    Ok(self.make_token("===", self.line, TokenKind::IdentityEquals))
                }
                [b'=', b'=', _res @ ..] => Ok(self.make_token("==", self.line, TokenKind::Equals_)),
                [b'=', b'>', _res @ ..] => Ok(self.make_token("=>", self.line, TokenKind::ARROW)),
                _ => Ok(self.make_token("=", self.line, TokenKind::Assign)),
            },
            b'&' => match self.bytes {
                [b'&', b'&', _res @ ..] => Ok(self.make_token("&&", self.line, TokenKind::And)),
                [b'&', b'=', _res @ ..] => {
                    Ok(self.make_token("&=", self.line, TokenKind::BitAndAssign))
                }
                _ => Ok(self.make_token("&", self.line, TokenKind::BitAnd)),
            },

            b'^' => match self.bytes {
                [b'^', b'=', _res @ ..] => {
                    Ok(self.make_token("^=", self.line, TokenKind::BitXorAssign))
                }
                _ => Ok(self.make_token("^", self.line, TokenKind::BitXOr)),
            },

            b'|' => match self.bytes {
                [b'|', b'=', _res @ ..] => {
                    Ok(self.make_token("|=", self.line, TokenKind::BitOrAssign))
                }
                [b'|', b'|', _res @ ..] => Ok(self.make_token("||", self.line, TokenKind::Or)),
                _ => Ok(self.make_token("|", self.line, TokenKind::BitOr)),
            },

            _ => Err(self.report_error("Unexpected character [Our compiler just supports ASCII]")),
        }
    }

    fn is_ws(&self) -> bool {
        if self.bytes.is_empty() {
            return false;
        }
        matches!(self.bytes[0], b' ' | b'\r' | b'\n' | b'\t')
    }

    fn is_comment(&self) -> bool {
        match self.bytes {
            [b'/', b'/' | b'*', _res @ ..] => true,
            _ => false,
        }
    }

    fn is_number(&self) -> bool {
        match self.bytes {
            [b'0'..=b'9', _res @ ..] => true,
            [b'.', b'0'..=b'9', _res @ ..] => true,
            _ => false,
        }
    }

    fn skip_unrelated(&mut self) {
        loop {
            match (self.is_ws(), self.is_comment()) {
                (false, false) => break,
                (true, false) => self.skip_ws(),
                (false, true) => self.skip_comment(),
                (true, true) => {
                    compiler_internal_error!(" Why it could be both whitespace and comment?")
                }
            }
        }
    }

    fn skip_ws(&mut self) {
        loop {
            match self.bytes.first() {
                Some(b'\n') | Some(b'\r') => {
                    self.inc_line();
                    self.forward(1);
                }
                Some(b' ') | Some(b'\t') => {
                    self.forward(1);
                }
                _ => break,
            }
        }
    }

    fn skip_comment(&mut self) {
        lazy_static! {
            static ref SINGLE_LINE_COMMENTS_RE: Regex = Regex::new(r"(^//.*)").unwrap();
            static ref MULTI_LINE_COMMENTS_RE: Regex = Regex::new(r"(^/[*][\s\S]*?[*]/)").unwrap();
        }

        loop {
            let src = str::from_utf8(self.bytes).unwrap_or("");
            let single_line_comments = SINGLE_LINE_COMMENTS_RE.captures(src);
            let multi_line_comments = MULTI_LINE_COMMENTS_RE.captures(src);
            match (single_line_comments, multi_line_comments) {
                (None, None) => break,
                (Some(single), None) => {
                    self.forward(single.get(1).unwrap().end());
                }
                (None, Some(multi)) => {
                    let len = multi.get(1).unwrap().end();
                    let mut slice = &self.bytes[..len];
                    loop {
                        match slice {
                            [b'\n' | b'\r', rest @ ..] => {
                                slice = rest;
                                self.line += 1
                            }
                            [_, rest @ ..] => {
                                slice = rest;
                            }
                            _ => break,
                        }
                    }
                    self.forward(len);
                }
                (Some(_), Some(_)) => compiler_internal_error!("Why it can be captures twice?"),
            }
        }
    }

    fn peek(&self) -> Option<u8> {
        match self.bytes.len() > 0 {
            true => Some(self.bytes[0]),
            false => None,
        }
    }

    fn make_token(&mut self, value: &'static str, line: usize, kind: TokenKind) -> Token {
        self.forward(value.len());
        Token::new(value, line, kind)
    }

    fn make_eof_token(&self, line: usize) -> Token {
        Token::new("$", line, TokenKind::EOF)
    }

    fn make_number_token(&mut self) -> Result<Token, LexerError> {
        lazy_static! {
            static ref HEX_RE: Regex = Regex::new(r"(^0[xX][0-9a-fA-F]+)").unwrap();
            static ref OCT_RE: Regex = Regex::new(r"(^0[oO][0-7]+)").unwrap();
            static ref OCT_RE2: Regex = Regex::new(r"(^0[0-7]+)").unwrap();
            static ref BIN_RE: Regex = Regex::new(r"(^0[bB][01]+)").unwrap();
        }

        let src = str::from_utf8(self.bytes);
        if src.is_err() {
            return Err(self.report_error("UFT8 ERROR"));
        }

        let src = src.unwrap();
        match self.bytes {
            [b'0', b'x' | b'X', _res @ ..] => match HEX_RE.captures(src) {
                Some(hex) => {
                    let hex = hex.get(1).unwrap().as_str();
                    self.forward(hex.len());
                    Ok(Token::new(hex, self.line, TokenKind::Number))
                }
                None => Err(self.report_error("unknown number character")),
            },
            [b'0', b'o' | b'O', _res @ ..] => match OCT_RE.captures(src) {
                Some(oct) => {
                    let oct = oct.get(1).unwrap().as_str();
                    self.forward(oct.len());
                    Ok(Token::new(oct, self.line, TokenKind::Number))
                }
                None => Err(self.report_error("unknown number character")),
            },
            [b'0', b'0'..=b'7', _res @ ..] => match OCT_RE2.captures(src) {
                Some(oct2) => {
                    let oct2 = oct2.get(1).unwrap().as_str();
                    self.forward(oct2.len());
                    Ok(Token::new(oct2, self.line, TokenKind::Number))
                }
                None => Err(self.report_error("unknown number character")),
            },
            [b'0', b'b' | b'B', _res @ ..] => match BIN_RE.captures(src) {
                Some(bin) => {
                    let bin = bin.get(1).unwrap().as_str();
                    self.forward(bin.len());
                    Ok(Token::new(bin, self.line, TokenKind::Number))
                }
                None => Err(self.report_error("unknown number character")),
            },
            _ => Ok(self.make_decimal_numbers_token(src)?),
        }
    }

    fn make_decimal_numbers_token(&mut self, src: &str) -> Result<Token, LexerError> {
        lazy_static! {
            static ref DCLINT_RE: &'static str = "0|[1-9][0-9]*";
            static ref EXP_RE: &'static str = "[eE][+-]?[0-9]+";
            static ref DECIMAL1: Regex =
                Regex::new(&format!(r"(^({0})[.][0-9]*({1})?)", *DCLINT_RE, *EXP_RE)).unwrap();
            static ref DECIMAL2: Regex =
                Regex::new(&format!(r"(^.[0-9]+({0})?)", *EXP_RE)).unwrap();
            static ref DECIMAL3: Regex =
                Regex::new(&format!(r"(^({0})({1})?)", *DCLINT_RE, *EXP_RE)).unwrap();
        }

        if self.peek().unwrap() == b'.' {
            match DECIMAL2.captures(src) {
                Some(decimal2) => {
                    let decimal2 = decimal2.get(1).unwrap().as_str();
                    self.forward(decimal2.len());
                    return Ok(Token::new(decimal2, self.line, TokenKind::Number));
                }
                None => return Err(self.report_error("unknown decimal")),
            }
        }

        match (DECIMAL1.captures(src), DECIMAL3.captures(src)) {
            (Some(decimal1), _) => {
                let decimal1 = decimal1.get(1).unwrap().as_str();
                self.forward(decimal1.len());
                return Ok(Token::new(decimal1, self.line, TokenKind::Number));
            }
            (None, Some(decimal3)) => {
                let decimal3 = decimal3.get(1).unwrap().as_str();
                self.forward(decimal3.len());
                return Ok(Token::new(decimal3, self.line, TokenKind::Number));
            }

            (None, None) => return Err(self.report_error("unknown decimal")),
        }
    }

    fn make_identifier_token(&mut self) -> Token {
        lazy_static! {
            static ref IDENTIFIER_RE: Regex = Regex::new(r"(^[_\d\w]+)").unwrap();
        }

        let src = str::from_utf8(self.bytes).unwrap_or("");
        let identifier = IDENTIFIER_RE.captures(src);
        match identifier {
            Some(identifier) => {
                let identifier = identifier.get(1).unwrap().as_str();
                self.forward(identifier.len());

                match KEYWORD.get(identifier) {
                    Some(&keyword) => Token::new(identifier, self.line, keyword),
                    None => Token::new(identifier, self.line, TokenKind::Identifier),
                }
            }
            None => compiler_internal_error!("Why it can be here?"),
        }
    }

    fn make_string_token(&mut self) -> Result<Token, LexerError> {
        let terminal = self.peek();
        let mut value = Vec::new();

        self.forward(1);
        loop {
            match self.peek() {
                // if the current character equals the terminal
                cur if cur == terminal => {
                    self.forward(1);
                    break;
                }

                // if the current character is a escaped character
                Some(b'\\') => {
                    todo!()
                }
                Some(b'\n') => {
                    self.forward(1);
                    self.inc_line();
                }
                Some(c) => {
                    value.push(c);
                    self.forward(1);
                }

                // if it has been the end of the string, that means error
                None => return Err(self.report_error("Unclosed string")),
            }
        }

        Ok(Token::new(
            str::from_utf8(&value).unwrap(),
            self.line,
            TokenKind::String,
        ))
    }

    fn forward(&mut self, n: usize) {
        match n <= self.bytes.len() {
            true => self.bytes = &self.bytes[n..],
            false => self.bytes = &self.bytes[self.bytes.len()..],
        }
    }

    fn inc_line(&mut self) {
        self.line += 1;
    }

    fn report_error(&self, s: &str) -> LexerError {
        LexerError::new(format!("Line[{}]: {}", self.line, s))
    }
}
