pub mod error;
mod exp_parser;

use crate::ast::ast_node::decorator::Decorators;
use crate::ast::ast_node::exp;
use crate::ast::ast_node::identifier;
use crate::ast::ast_node::identifier::Identifier;
use crate::ast::ast_node::literal::Literal;
use crate::ast::ast_node::program::Program;

use crate::ast::visulize::Visualizable;
use crate::ast::ASTNode;
use lazy_static::lazy_static;

use crate::ast::ast_node::block::*;
use crate::ast::ast_node::body::*;
use crate::ast::ast_node::case::*;
use crate::ast::ast_node::class::*;
use crate::ast::ast_node::clause::*;
use crate::ast::ast_node::decl::*;
use crate::ast::ast_node::exp::*;
use crate::ast::ast_node::parameter::*;
use crate::ast::ast_node::sig::*;
use crate::ast::ast_node::source_element::*;
use crate::ast::ast_node::stat::*;
use crate::ast::ast_node::type_::*;

use crate::ast::ast_node::unknown::Unknown;
use crate::compiler_internal_error;
use crate::lexer::token::Token;
use crate::lexer::token_kind::{KeyWordKind, TokenKind};
use crate::{ast::AST, error::TSError};

use self::error::ParserError;

pub(crate) struct Parser {
    filename: String,
    tokens: Vec<Token>,
    index: usize,

    error_most_possible: Option<ParserError>,
    try_most_forward: usize,
}
impl Parser {
    pub(crate) fn new(tokens: Vec<Token>, filename: &str) -> Self {
        Self {
            filename: filename.to_owned(),
            tokens,
            index: 0,
            error_most_possible: None,
            try_most_forward: 0,
        }
    }

    pub(crate) fn show_tokens(&mut self) {
        for token in &self.tokens {
            println!("{}", token);
        }
    }

    // general error report
    fn report_error(&mut self, msg: &str) -> ParserError {
        let cur = self.peek().unwrap();
        if self.index < self.try_most_forward {
            // 拦截 msg, 换上更准确的错误。
            // 但是实际上，更准确的那个错误也是从 else 的那个分支传出来的
            return self.error_most_possible.clone().unwrap();
        }

        ParserError::new(format!(
            "{}: SyntaxError: near Line[{}]: {}",
            self.filename,
            cur.peek_line(),
            msg
        ))
    }

    // expect token error
    fn expect_error(&mut self, stat: &str, expects: &str) -> ParserError {
        let cur = self.peek().unwrap();
        self.report_error(&format!(
            "{}: Expect [{}] but got token [ {} ] ({})",
            stat,
            expects,
            cur.peek_value(),
            cur.peek_kind()
        ))
    }

    fn unsupported_error(&mut self, unsupported: &str) -> ParserError {
        self.report_error(&format!("Sorry, but now {} is not supported", unsupported))
    }

    fn eat(&mut self, kind: TokenKind) -> Result<(), ParserError> {
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

    fn is_eos(&mut self) -> bool {
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

    fn eat_eos(&mut self) -> Result<(), ParserError> {
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

    fn is_literal(&self) -> bool {
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

    /*
    尝试函数，选择一个分支进行尝试，成功则返回，出错则回溯

    没事别用 try_to, 用 try_to 必须是在不确定分支选择的时候
    try_to 是有限个前瞻无法解决的时候使用，即处理 LL(*) 时采用。
    凡是有限个前瞻可以解决的，不适用 try_to
    有 try_to 的地方必须有所有候选分支都不匹配的 Err
    */
    fn try_to<T: Visualizable>(
        &mut self,
        // 函数指针大小固定为一个指针大小
        func: fn(&mut Parser) -> Result<ASTNode<T>, ParserError>,
    ) -> Option<ASTNode<T>> {
        let current = self.index;
        match func(self) {
            Ok(stat) => {
                if self.index > self.try_most_forward {
                    // 清除之前的错误
                    self.try_most_forward = 0;
                    self.error_most_possible = None;
                }

                Some(stat)
            }
            Err(err) => {
                // 记录走到最远处所触发的的错误
                if self.index > self.try_most_forward {
                    self.try_most_forward = self.index;
                    self.error_most_possible = Some(err);
                }
                self.index = current;
                None
            }
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    // 注意，该函数在 extract 的同时也会 eat Token
    fn extact_identifier(&mut self) -> Result<String, ParserError> {
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
    fn extact_literal(&mut self) -> Result<Literal, ParserError> {
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

    fn peek_kind(&self) -> TokenKind {
        match self.peek() {
            Some(token) => token.peek_kind(),
            None => TokenKind::EOF,
        }
    }

    fn look_ahead(&self) -> Option<TokenKind> {
        match self.tokens.get(self.index + 1) {
            Some(token) => Some(token.peek_kind()),
            None => None,
        }
    }

    fn look_ahead2(&self) -> Option<TokenKind> {
        match self.tokens.get(self.index + 2) {
            Some(token) => Some(token.peek_kind()),
            None => None,
        }
    }

    fn look_ahead3(&self) -> Option<TokenKind> {
        match self.tokens.get(self.index + 3) {
            Some(token) => Some(token.peek_kind()),
            None => None,
        }
    }

    fn is_new_line(&self) -> bool {
        if let (Some(current), Some(pre)) =
            (self.tokens.get(self.index), self.tokens.get(self.index - 1))
        {
            current.peek_line() > pre.peek_line()
        } else {
            false
        }
    }

    fn kind_is(&self, kind: TokenKind) -> bool {
        match self.peek() {
            Some(token) => token.kind_is(kind),
            None => false,
        }
    }

    fn pre_peek_kind(&self) -> TokenKind {
        let pre_token = self.tokens.get(self.index - 1);
        match pre_token {
            Some(pre_token) => pre_token.peek_kind(),
            None => TokenKind::EOF,
        }
    }

    fn prekind_is(&self, kind: TokenKind) -> bool {
        let pre_token = &self.tokens[self.index - 1];
        return pre_token.kind_is(kind);
    }

    pub(crate) fn parse(&mut self) -> Result<AST, TSError> {
        Ok(AST::new(self.parse_program()?))
    }

    fn parse_program(&mut self) -> Result<Program, ParserError> {
        let mut programe = Program::new();

        match self.kind_is(TokenKind::EOF) {
            true => Ok(programe),
            false => {
                programe.set_source_elements(self.parse_source_elements()?);
                Ok(programe)
            }
        }
    }

    // sourceElements: sourceElement+;
    fn parse_source_elements(&mut self) -> Result<ASTNode<SourceElements>, ParserError> {
        let mut source_elements = SourceElements::default();

        loop {
            let source_element = self.parse_stat()?;
            source_elements.push_stat(source_element);

            // sourceElement 只可能有两个 follow: { EOF, "}" }
            if self.kind_is(TokenKind::EOF) || self.kind_is(TokenKind::RightBracket) {
                break;
            }
        }
        Ok(ASTNode::new(source_elements))
    }

    fn parse_stat(&mut self) -> Result<ASTNode<Stat>, ParserError> {
        match self.peek_kind() {
            TokenKind::LeftBracket => Ok(ASTNode::new(Stat::Block(self.parse_block()?))),
            TokenKind::KeyWord(KeyWordKind::Import) => {
                Ok(ASTNode::new(Stat::ImportStat(self.parse_import_stat()?)))
            }
            TokenKind::KeyWord(KeyWordKind::Export) => match self.look_ahead() {
                Some(TokenKind::KeyWord(KeyWordKind::Declare))
                | Some(TokenKind::KeyWord(KeyWordKind::Interface)) => Ok(ASTNode::new(
                    Stat::InterfaceDecl(self.parse_interface_decl()?),
                )),

                _ => Ok(ASTNode::new(Stat::ExportStat(self.parse_export_stat()?))),
            },
            TokenKind::SemiColon => Ok(ASTNode::new(Stat::EmptyStat(self.parse_empty_stat()?))),

            // abstract class or abstract ?
            TokenKind::KeyWord(KeyWordKind::Abstract) => match self.look_ahead() {
                Some(TokenKind::KeyWord(KeyWordKind::Class)) => {
                    Ok(ASTNode::new(Stat::ClassDecl(self.parse_class_decl()?)))
                }
                _ => Ok(ASTNode::new(Stat::AbsDecl(self.parse_abstract_decl()?))),
            },

            TokenKind::KeyWord(KeyWordKind::Class) => {
                Ok(ASTNode::new(Stat::ClassDecl(self.parse_class_decl()?)))
            }

            TokenKind::KeyWord(KeyWordKind::Interface) => Ok(ASTNode::new(Stat::InterfaceDecl(
                self.parse_interface_decl()?,
            ))),

            TokenKind::KeyWord(KeyWordKind::Namespace) => Ok(ASTNode::new(Stat::NamespaceDecl(
                self.parse_namespace_decl()?,
            ))),

            TokenKind::KeyWord(KeyWordKind::If) => {
                Ok(ASTNode::new(Stat::IfStat(self.parse_if_stat()?)))
            }

            // do|while|for -> iteration stat
            TokenKind::KeyWord(KeyWordKind::Do)
            | TokenKind::KeyWord(KeyWordKind::While)
            | TokenKind::KeyWord(KeyWordKind::For) => {
                Ok(ASTNode::new(Stat::IterStat(self.parse_iter_stat()?)))
            }

            // TokenKind::KeyWord(KeyWordKind::Continue) => Ok(Some(self.parse_continue_stat()?)),
            // TokenKind::KeyWord(KeyWordKind::Break) => Ok(Some(self.parse_break_stat()?)),
            TokenKind::KeyWord(KeyWordKind::Return) => Ok(ASTNode::new(Stat::ReturnStat(self.parse_return_stat()?))),
            // TokenKind::KeyWord(KeyWordKind::Yield) => Ok(Some(self.parse_yield_stat()?)),
            // TokenKind::KeyWord(KeyWordKind::With) => Ok(Some(self.parse_with_stat()?)),

            // TokenKind::Identifier => match self.lookAhead() {
            //     Some(TokenKind::Colon) => Ok(Some(self.parse_labelled_stat()?)),

            //     _ => Ok(Some(self.parse_abstract_declaration()?)),
            // },

            // TokenKind::KeyWord(KeyWordKind::Switch) => Ok(Some(self.parse_switch_stat()?)),
            // TokenKind::KeyWord(KeyWordKind::Throw) => Ok(Some(self.parse_throw_stat()?)),
            // TokenKind::KeyWord(KeyWordKind::Try) => Ok(Some(self.parse_try_stat()?)),
            // TokenKind::KeyWord(KeyWordKind::Debugger) => Ok(Some(self.parse_debugger_stat()?)),
            // TokenKind::KeyWord(KeyWordKind::Function_) => match self.look_ahead() {
            //     Some(TokenKind::Identifier) => match self.parse_func_declaration()? {
            //         Some(func_decl) => Ok(Some(ASTNode::new(Stat::FuncDecl(func_decl)))),
            //         None => Ok(None),
            //     },
            //     Some(TokenKind::LeftParen) => match self.parse_func_exp_declaration()? {
            //         Some(func_exp_decl) => Ok(Some(ASTNode::new(Stat::FuncExpDecl(func_exp_decl)))),
            //         None => Ok(None),
            //     },
            //     Some(TokenKind::Multiply) => match self.parse_generator_func_declaration()? {
            //         Some(gen_func_decl) => Ok(Some(ASTNode::new(Stat::GenFuncDecl(gen_func_decl)))),
            //         None => Ok(None),
            //     },
            //     _ => Err(self.expect_error("Func", "Identifier, ( or *")),
            // },

            // todo how to deal with arrow functions
            // todo how to deal with variable statement
            // todo how to deal with type aliases
            // todo how to deal with enum declarations

            // 字面量
            TokenKind::Identifier
            | TokenKind::LeftParen
            | TokenKind::KeyWord(KeyWordKind::This)
            | TokenKind::KeyWord(KeyWordKind::Super)
            | TokenKind::KeyWord(KeyWordKind::New)
            | TokenKind::KeyWord(KeyWordKind::Delete)
            | TokenKind::KeyWord(KeyWordKind::Typeof)
            // literal
            | TokenKind::String
            | TokenKind::Number
            | TokenKind::KeyWord(KeyWordKind::True)
            | TokenKind::KeyWord(KeyWordKind::False)
            | TokenKind::KeyWord(KeyWordKind::Null) => {
                let exp_stat = self.parse_exp_seq()?;
                if self.kind_is(TokenKind::SemiColon) {
                    self.eat(TokenKind::SemiColon)?;
                }
                if self.kind_is(TokenKind::Identifier) && ! self.is_new_line() {
                    return Err(self.report_error("maybe you forgot [,] to separate expression ?"))
                }
                Ok(ASTNode::new(Stat::ExpStat(exp_stat)))
            }

            // TokenKind::Number
            // | TokenKind::String
            // | TokenKind::KeyWord(KeyWordKind::True)
            // | TokenKind::KeyWord(KeyWordKind::False)
            // | TokenKind::KeyWord(KeyWordKind::Null) => {
            //     let exp_stat = self.parse_exp_seq()?;
            //     if self.kind_is(TokenKind::SemiColon) {
            //         self.eat(TokenKind::SemiColon)?;
            //     }
            //     Ok(ASTNode::new(Stat::ExpStat(exp_stat)))
            // }
            _ => {
                Err(self.report_error(&format!("Stat: Unexpected Token {}", self.peek().unwrap())))
            }
        }
    }

    fn parse_block(&mut self) -> Result<ASTNode<Block>, ParserError> {
        let mut block = Block::default();
        self.eat(TokenKind::LeftBracket)?;
        loop {
            if self.kind_is(TokenKind::RightBracket) {
                break;
            }
            let stat = self.parse_stat()?;
            block.push(stat);
        }
        self.eat(TokenKind::RightBracket)?;
        Ok(ASTNode::new(block))
    }

    /*
    格式: (* | ( a | a, {b as c, ...})) from yyy
    * 可以 as, 但是 a 不能 as, {} 块内部可以 as

    fromBlock: (Multiply (As Identifier)? | multipleImportStatement) From StringLiteral eos;
    multipleImportStatement:
        Identifier
        | (Identifier ',')? '{' Import* '}';
    Import: Identifier (As Identifier)?;
    */
    fn parse_import_stat(&mut self) -> Result<ASTNode<ImportStat>, ParserError> {
        let mut import_stat = ImportStat::default();
        self.eat(TokenKind::KeyWord(KeyWordKind::Import))?;
        import_stat.set_from_block(self.parse_from_block()?);
        Ok(ASTNode::new(import_stat))
    }

    fn parse_from_block(&mut self) -> Result<ASTNode<FromBlock>, ParserError> {
        let mut from_block = FromBlock::default();
        match self.peek_kind() {
            TokenKind::Multiply => {
                from_block.set_all();
                self.eat(TokenKind::Multiply)?;
                if self.kind_is(TokenKind::KeyWord(KeyWordKind::As)) {
                    self.eat(TokenKind::KeyWord(KeyWordKind::As))?;
                    match self.peek_kind() {
                        TokenKind::Identifier => {
                            from_block.set_all_alias(&self.extact_identifier()?);
                            if self.kind_is(TokenKind::Comma) {
                                self.eat(TokenKind::Comma)?;
                            }
                        }
                        _ => return Err(self.expect_error("Import Statement", "Identifier")),
                    }
                }
            }
            TokenKind::Identifier | TokenKind::LeftBracket => {
                if self.kind_is(TokenKind::Identifier) {
                    from_block.set_imported(&self.extact_identifier()?);

                    if self.kind_is(TokenKind::Comma) {
                        self.eat(TokenKind::Comma)?;
                    }
                }
                // if it be "{a, b as c, ...}"
                if self.kind_is(TokenKind::LeftBracket) {
                    self.eat(TokenKind::LeftBracket)?;
                    while self.kind_is(TokenKind::Identifier) {
                        let imported = self.extact_identifier()?;

                        // self.eat(TokenKind::Identifier)?;
                        match self.kind_is(TokenKind::KeyWord(KeyWordKind::As)) {
                            true => {
                                self.eat(TokenKind::KeyWord(KeyWordKind::As))?;
                                match self.peek_kind() {
                                    TokenKind::Identifier => {
                                        from_block.push_imported_alias(
                                            &imported,
                                            Some(&self.extact_identifier()?),
                                        );
                                    }
                                    _ => {
                                        return Err(
                                            self.expect_error("Import Statement", "Identifier")
                                        )
                                    }
                                }
                            }
                            false => {
                                from_block.push_imported_alias(&imported, None);
                            }
                        }

                        if self.kind_is(TokenKind::Comma) {
                            self.eat(TokenKind::Comma)?;
                        }
                    }
                    self.eat(TokenKind::RightBracket)?;
                }
            }
            _ => return Err(self.expect_error("Import Statement", "* or Identifier")),
        }

        self.eat(TokenKind::KeyWord(KeyWordKind::From))?;
        match self.peek_kind() {
            TokenKind::String => {
                from_block.set_from_value(self.peek().unwrap().peek_value());
                self.eat(TokenKind::String)?;
            }
            _ => return Err(self.expect_error("Import Statement", "String Literal")),
        }

        self.eat_eos()?;
        Ok(ASTNode::new(from_block))
    }

    /*
        exportStatement: Export Default? (fromBlock | statement);
    */
    fn parse_export_stat(&mut self) -> Result<ASTNode<ExportStat>, ParserError> {
        let mut export_stat = ExportStat::default();

        self.eat(TokenKind::KeyWord(KeyWordKind::Export))?;
        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Default)) {
            export_stat.set_default();
            self.eat(TokenKind::KeyWord(KeyWordKind::Default))?;
        }

        // 此处进行 corner case 处理
        match self.peek_kind() {
            // 不允许 export [default] export [default] export ... 这样的循环嵌套
            TokenKind::KeyWord(KeyWordKind::Export) => {
                return Err(self.report_error("export [default] export?  Damn you !!!"));
            }

            // 不允许直接 export;
            TokenKind::SemiColon => {
                return Err(self.expect_error("Export Stat", "FromBlock or Statement"));
            }
            _ => (),
        }

        // 尝试性地解析 from block
        if let Some(from_block) = self.try_to(Parser::parse_from_block) {
            export_stat.set_from_block(from_block);
            return Ok(ASTNode::new(export_stat));
        }

        // 如果不是 from block, 那么说明一定是 stat
        // 之前的 match 保证进入这里面的 stat 一定不是 export 开头
        if let Some(from_block) = self.try_to(Parser::parse_stat) {
            export_stat.set_stat(from_block);
            self.eat_eos()?;
            return Ok(ASTNode::new(export_stat));
        }

        // 两个都不是, 出错
        Err(self.report_error("Expect [FromBlock] or [Statment] but there is no such match"))
    }

    fn parse_empty_stat(&mut self) -> Result<ASTNode<EmptyStat>, ParserError> {
        self.eat(TokenKind::SemiColon)?;
        Ok(ASTNode::new(EmptyStat::new()))
    }

    /*
    classDeclaration:
        Abstract? Class Identifier typeParameters? classHeritage classTail;

    for now typeParameters is not supported
    */
    fn parse_class_decl(&mut self) -> Result<ASTNode<ClassDecl>, ParserError> {
        let mut class_decl = ClassDecl::default();
        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Abstract)) {
            class_decl.set_abstract();
            self.eat(TokenKind::KeyWord(KeyWordKind::Abstract))?;
        }
        self.eat(TokenKind::KeyWord(KeyWordKind::Class))?;
        match self.peek_kind() {
            TokenKind::Identifier => {
                class_decl.set_class_name(&self.extact_identifier()?);
            }
            _ => return Err(self.expect_error("ClassDecl Stat", "Identifer(class name)")),
        }

        if self.kind_is(TokenKind::LessThan) {
            class_decl.set_type_paras(self.parse_type_paras()?);
        }

        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Extends))
            || self.kind_is(TokenKind::KeyWord(KeyWordKind::Implements))
        {
            class_decl.set_class_heritage(self.parse_class_heritage()?);
        }
        class_decl.set_class_tail(self.parse_class_tail()?);
        Ok(ASTNode::new(class_decl))
    }

    fn parse_class_heritage(&mut self) -> Result<ASTNode<ClassHeritage>, ParserError> {
        let mut class_heritage = ClassHeritage::default();
        match self.peek_kind() {
            TokenKind::KeyWord(KeyWordKind::Extends)
            | TokenKind::KeyWord(KeyWordKind::Implements) => {
                if self.kind_is(TokenKind::KeyWord(KeyWordKind::Extends)) {
                    self.eat(TokenKind::KeyWord(KeyWordKind::Extends))?;
                    let extended_type = self.parse_type_ref()?;
                    class_heritage.set_extends(Extends::new(extended_type));
                }
                if self.kind_is(TokenKind::KeyWord(KeyWordKind::Implements)) {
                    let mut implemented = Implement::default();
                    self.eat(TokenKind::KeyWord(KeyWordKind::Implements))?;
                    loop {
                        let type_ref = self.parse_type_ref()?;
                        implemented.push_implemented(type_ref);
                        match self.peek_kind() {
                            TokenKind::Comma => {
                                self.eat(TokenKind::Comma)?;
                            }
                            _ => break,
                        }
                    }
                    class_heritage.set_implement(implemented);
                }
                Ok(ASTNode::new(class_heritage))
            }
            _ => return Err(self.expect_error("ClassHeritage Stat", "Extends or implements")),
        }
    }

    fn parse_type_ref(&mut self) -> Result<ASTNode<TypeRef>, ParserError> {
        let mut type_ref = TypeRef::default();

        match self.peek_kind() {
            TokenKind::Identifier => {
                type_ref.set_type_name(&self.extact_identifier()?);
                if self.kind_is(TokenKind::LessThan) {
                    type_ref.set_type_generic(self.parse_type_generic()?);
                }
                Ok(ASTNode::new(type_ref))
            }
            _ => Err(self.expect_error("Type Ref", "Identifier")),
        }
    }

    /*
    typeGeneric: '<' typeArgumentList '>';
    typeArgumentList: typeArgument (',' typeArgument)*;
    */
    fn parse_type_generic(&mut self) -> Result<ASTNode<TypeGeneric>, ParserError> {
        Err(self.unsupported_error("Type Generic"))
    }

    /*
    classTail: '{' classElement* '}';
    */
    fn parse_class_tail(&mut self) -> Result<ASTNode<ClassTail>, ParserError> {
        let mut class_tail = ClassTail::default();
        self.eat(TokenKind::LeftBracket)?;

        if self.kind_is(TokenKind::RightBracket) {
            self.eat(TokenKind::RightBracket)?;
            return Ok(ASTNode::new(class_tail));
        }

        loop {
            let class_element = self.parse_class_element()?;
            class_tail.push_class_element(class_element);
            if self.kind_is(TokenKind::RightBracket) {
                self.eat(TokenKind::RightBracket)?;
                return Ok(ASTNode::new(class_tail));
            }
        }
    }

    /*
    classElement:
        constructorDeclaration
        | propertyMemberDeclaration
        | indexMemberDeclaration;

    constructorDeclaration 第一个是访问修饰符, 第二个必定是 constructor 关键字
    indexMemberDeclaration  以 [ 开头
    */
    fn parse_class_element(&mut self) -> Result<ASTNode<ClassElement>, ParserError> {
        match self.peek_kind() {
            // constructorDeclaration
            TokenKind::KeyWord(KeyWordKind::Constructor) => Ok(ASTNode::new(
                ClassElement::ConstructorDecl(self.parse_cons_decl()?),
            )),

            // propertyMemberDeclaration
            TokenKind::Identifier => Ok(ASTNode::new(ClassElement::PropertyMemberDecl(
                self.parse_property_member_decl()?,
            ))),

            // indexMemberDeclaration
            TokenKind::LeftBrace => Ok(ASTNode::new(ClassElement::IndexMemberDecl(
                self.parse_index_member_decl()?,
            ))),

            // propertyMemberDeclaration
            TokenKind::KeyWord(KeyWordKind::Async)
            | TokenKind::KeyWord(KeyWordKind::Static)
            | TokenKind::KeyWord(KeyWordKind::ReadOnly)
            | TokenKind::KeyWord(KeyWordKind::Abstract) => Ok(ASTNode::new(
                ClassElement::PropertyMemberDecl(self.parse_property_member_decl()?),
            )),

            TokenKind::KeyWord(KeyWordKind::Public)
            | TokenKind::KeyWord(KeyWordKind::Private)
            | TokenKind::KeyWord(KeyWordKind::Protected) => match self.look_ahead() {
                // constructorDeclaration
                Some(TokenKind::KeyWord(KeyWordKind::Constructor)) => Ok(ASTNode::new(
                    ClassElement::ConstructorDecl(self.parse_cons_decl()?),
                )),

                // propertyMemberDeclaration
                _ => Ok(ASTNode::new(ClassElement::PropertyMemberDecl(
                    self.parse_property_member_decl()?,
                ))),
            },

            _ => {
                return Err(self.expect_error(
                    "Class Element",
                    "constructorDeclaration or propertyMemberDeclaration or indexMemberDeclaration",
                ))
            }
        }
    }

    /*
    constructorDeclaration:
        accessibilityModifier? Constructor '(' formalParameterList? ')' '{' functionBody '}';
    */
    fn parse_cons_decl(&mut self) -> Result<ASTNode<ConstructorDecl>, ParserError> {
        let mut cons_decl = ConstructorDecl::default();

        match self.peek_kind() {
            TokenKind::KeyWord(KeyWordKind::Public)
            | TokenKind::KeyWord(KeyWordKind::Private)
            | TokenKind::KeyWord(KeyWordKind::Protected) => {
                cons_decl.set_access_modifier(self.parse_access_modifier()?)
            }

            _ => (),
        }

        self.eat(TokenKind::KeyWord(KeyWordKind::Constructor))?;

        self.eat(TokenKind::LeftParen)?;
        if !self.kind_is(TokenKind::RightParen) {
            cons_decl.set_formal_paras(self.parse_formal_parameters()?);
        }
        self.eat(TokenKind::RightParen)?;

        self.eat(TokenKind::LeftBracket)?;
        cons_decl.set_func_body(self.parse_func_body()?);
        self.eat(TokenKind::RightBracket)?;
        Ok(ASTNode::new(cons_decl))
    }

    /*
    propertyMemberDeclaration:

    accessibilityModifier? Static? ReadOnly? Identifier '?'? typeAnnotation? '=' singleExpression? SemiColon #
        PropertyDeclarationExpression
    | accessibilityModifier? Static? Async? Identifier callSignature ( ('{' functionBody '}') | SemiColon )													# MethodDeclarationExpression
    | accessibilityModifier? Static? (getAccessor | setAccessor)	# GetterSetterDeclarationExpression
    | abstractDeclaration								# AbstractMemberDeclaration;
    ;
    */
    fn parse_property_member_decl(&mut self) -> Result<ASTNode<PropertyMemberDecl>, ParserError> {
        match self.peek_kind() {
            TokenKind::KeyWord(KeyWordKind::Abstract) => {
                // abstractDeclaration
                return Ok(ASTNode::new(PropertyMemberDecl::AbsMemberDecl(
                    self.parse_abstract_decl()?,
                )));
            }

            _ => {
                if let Some(property_decl_exp) = self.try_to(Parser::parse_property_decl_exp) {
                    return Ok(ASTNode::new(PropertyMemberDecl::PropertyDeclExp(
                        property_decl_exp,
                    )));
                }

                if let Some(method_declaration_exp) = self.try_to(Parser::parse_method_decl_exp) {
                    return Ok(ASTNode::new(PropertyMemberDecl::MethodDeclExp(
                        method_declaration_exp,
                    )));
                }

                if let Some(gettersetter_decl_exp) =
                    self.try_to(Parser::parse_gettersetter_decl_exp)
                {
                    return Ok(ASTNode::new(PropertyMemberDecl::GetterSetterDeclExp(
                        gettersetter_decl_exp,
                    )));
                }

                Err(self.expect_error(
                    "Property Member Decl",
                    "PropertyDeclExp or MethodDeclExp or GetterSetterDeclExp or AbsMemberDecl",
                ))
            }
        }
    }

    /*
    accessibilityModifier? Static? ReadOnly? Identifier '?'? typeAnnotation? '=' singleExpression? SemiColon
    */
    fn parse_property_decl_exp(&mut self) -> Result<ASTNode<PropertyDeclExp>, ParserError> {
        let mut property_decl_exp = PropertyDeclExp::default();

        if let Some(access_modifier) = self.try_to(Parser::parse_access_modifier) {
            property_decl_exp.set_access_modifier(access_modifier);
        }

        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Static)) {
            self.eat(TokenKind::KeyWord(KeyWordKind::Static))?;
            property_decl_exp.set_static();
        }
        if self.kind_is(TokenKind::KeyWord(KeyWordKind::ReadOnly)) {
            self.eat(TokenKind::KeyWord(KeyWordKind::ReadOnly))?;
            property_decl_exp.set_readonly();
        }

        property_decl_exp.set_identifier(&self.extact_identifier()?);

        if self.kind_is(TokenKind::QuestionMark) {
            property_decl_exp.set_question_mark();
            self.eat(TokenKind::QuestionMark)?;
        }

        if self.kind_is(TokenKind::Colon) {
            property_decl_exp.set_type_annotation(self.parse_type_annotation()?);
        }

        if self.kind_is(TokenKind::Assign) {
            self.eat(TokenKind::Assign)?;
            property_decl_exp.set_initializer(self.parse_exp()?);
        }

        self.eat(TokenKind::SemiColon)?;

        Ok(ASTNode::new(property_decl_exp))
    }

    /*
    accessibilityModifier? Static? Async? Identifier callSignature ( ('{' functionBody '}') | SemiColon )
        */
    fn parse_method_decl_exp(&mut self) -> Result<ASTNode<MethodDeclExp>, ParserError> {
        let mut method_decl_exp = MethodDeclExp::default();

        if let Some(access_modifier) = self.try_to(Parser::parse_access_modifier) {
            method_decl_exp.set_access_modifier(access_modifier);
        }

        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Static)) {
            self.eat(TokenKind::KeyWord(KeyWordKind::Static))?;
            method_decl_exp.set_static();
        }
        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Async)) {
            self.eat(TokenKind::KeyWord(KeyWordKind::Async))?;
            method_decl_exp.set_async();
        }

        method_decl_exp.set_identifier(&self.extact_identifier()?);
        method_decl_exp.set_call_sig(self.parse_call_sig()?);

        match self.peek_kind() {
            TokenKind::LeftBracket => {
                self.eat(TokenKind::LeftBracket)?;
                method_decl_exp.set_func_body(self.parse_func_body()?);
                self.eat(TokenKind::RightBracket)?;
            }
            TokenKind::SemiColon => return Ok(ASTNode::new(method_decl_exp)),
            _ => {
                return Err(self.expect_error("Method Declaration Expression", "{ funcbody } or ;"));
            }
        }

        Ok(ASTNode::new(method_decl_exp))
    }

    /*
    accessibilityModifier? Static? (getAccessor | setAccessor)
    */
    fn parse_gettersetter_decl_exp(&mut self) -> Result<ASTNode<GetterSetterDeclExp>, ParserError> {
        // it is ugly, but it is necessary
        let mut access_modifier_ = None;
        let mut static_ = false;
        if let Some(access_modifier) = self.try_to(Parser::parse_access_modifier) {
            access_modifier_ = Some(access_modifier);
        }
        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Static)) {
            static_ = true;
            self.eat(TokenKind::KeyWord(KeyWordKind::Static))?;
        }

        let getter_setter_decl_exp =
            GetterSetterDeclExp::new(access_modifier_, static_, self.parse_accesser()?);
        Ok(ASTNode::new(getter_setter_decl_exp))
    }

    fn parse_accesser(&mut self) -> Result<ASTNode<Accesser>, ParserError> {
        match self.peek_kind() {
            TokenKind::KeyWord(KeyWordKind::Get) => {
                let mut accesser = GetAccesser::default();
                self.eat(TokenKind::KeyWord(KeyWordKind::Get))?;
                accesser.set_identifier(&self.extact_identifier()?);
                self.eat(TokenKind::LeftParen)?;
                self.eat(TokenKind::RightParen)?;
                if self.kind_is(TokenKind::Colon) {
                    accesser.set_type_annotation(self.parse_type_annotation()?);
                }
                self.eat(TokenKind::LeftBracket)?;
                accesser.set_func_body(self.parse_func_body()?);
                self.eat(TokenKind::RightBracket)?;
                Ok(ASTNode::new(Accesser::GetAccessor(accesser)))
            }
            TokenKind::KeyWord(KeyWordKind::Set) => {
                let mut accesser = SetAccesser::default();
                self.eat(TokenKind::KeyWord(KeyWordKind::Set))?;
                accesser.set_identifier(&self.extact_identifier()?);
                self.eat(TokenKind::LeftParen)?;
                accesser.set_parameter(&self.extact_identifier()?);
                if self.kind_is(TokenKind::Colon) {
                    accesser.set_type_annotation(self.parse_type_annotation()?);
                }
                self.eat(TokenKind::RightParen)?;
                self.eat(TokenKind::LeftBracket)?;
                accesser.set_func_body(self.parse_func_body()?);
                self.eat(TokenKind::RightBracket)?;
                Ok(ASTNode::new(Accesser::SetAccessor(accesser)))
            }
            _ => {
                return Err(self.expect_error(
                    "Getter/Setter Declaration Expression",
                    "{ getAccessor | setAccessor }",
                ));
            }
        }
    }

    /*
    indexMemberDeclaration: indexSignature SemiColon;
    */
    fn parse_index_member_decl(&mut self) -> Result<ASTNode<IndexMemberDecl>, ParserError> {
        let index_sig = self.parse_index_sig()?;
        self.eat(TokenKind::SemiColon)?;
        Ok(ASTNode::new(IndexMemberDecl::new(index_sig)))
    }

    /*
    indexSignature:
        '[' Identifier ':' (Number | String) ']' typeAnnotation;
    */
    fn parse_index_sig(&mut self) -> Result<ASTNode<IndexSig>, ParserError> {
        let type_;
        self.eat(TokenKind::LeftBrace)?;
        let index_name = &self.extact_identifier()?;
        self.eat(TokenKind::Colon)?;
        match self.peek_kind() {
            TokenKind::KeyWord(KeyWordKind::Number) => {
                type_ = Some(ASTNode::new(PredefinedType::Number));
                self.eat(TokenKind::KeyWord(KeyWordKind::Number))?;
            }
            TokenKind::KeyWord(KeyWordKind::String) => {
                type_ = Some(ASTNode::new(PredefinedType::String));
                self.eat(TokenKind::KeyWord(KeyWordKind::String))?;
            }
            _ => return Err(self.expect_error("Index Signature", "Number or String")),
        };
        self.eat(TokenKind::RightBrace)?;
        let type_annotation = self.parse_type_annotation()?;

        Ok(ASTNode::new(IndexSig::new(
            index_name,
            type_,
            type_annotation,
        )))
    }

    /*
        abstractDeclaration: Abstract Identifier callSignature eos;
    */
    fn parse_abstract_decl(&mut self) -> Result<ASTNode<AbsDecl>, ParserError> {
        let mut abs_decl = AbsDecl::default();
        self.eat(TokenKind::KeyWord(KeyWordKind::Abstract))?;
        abs_decl.set_identifier(&self.extact_identifier()?);
        abs_decl.set_call_sig(self.parse_call_sig()?);
        self.eat_eos()?;
        Ok(ASTNode::new(abs_decl))
    }

    /*
    ifStatement:
        If '(' expressionSequence ')' statement (Else statement)?;
    */
    fn parse_if_stat(&mut self) -> Result<ASTNode<IfStat>, ParserError> {
        let mut if_stat = IfStat::default();
        self.eat(TokenKind::KeyWord(KeyWordKind::If))?;

        self.eat(TokenKind::LeftParen)?;
        if_stat.set_exp_seq(self.parse_exp_seq()?);
        self.eat(TokenKind::RightParen)?;

        if_stat.set_stat(self.parse_stat()?);

        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Else)) {
            self.eat(TokenKind::KeyWord(KeyWordKind::Else))?;
            if_stat.set_else_stat(self.parse_stat()?);
        }
        Ok(ASTNode::new(if_stat))
    }

    /*
    iterationStatement:
        Do statement While '(' singleExpression ')' eos	# DoStatement
        | While '(' singleExpression ')' statement		# WhileStatement
        | For '(' expressionSequence? SemiColon singleExpression? SemiColon expressionSequence? ')'
            statement # ForStatement
        | For '(' varModifier variableDeclarationList SemiColon singleExpression? SemiColon
            expressionSequence? ')' statement							# ForVarStatement
        | For '(' singleExpression In singleExpression ')' statement	# ForInStatement;

        varModifier:Var | Let | Const;

    */
    fn parse_iter_stat(&mut self) -> Result<ASTNode<IterStat>, ParserError> {
        // now thing to do
        match self.peek_kind() {
            // Do statement While '(' singleExpression ')' eos	# DoStatement
            TokenKind::KeyWord(KeyWordKind::Do) => {
                self.eat(TokenKind::KeyWord(KeyWordKind::Do))?;
                let stat = self.parse_stat()?;
                self.eat(TokenKind::KeyWord(KeyWordKind::While))?;
                self.eat(TokenKind::LeftParen)?;
                let exp = self.parse_exp()?;
                self.eat(TokenKind::RightParen)?;
                self.eat_eos()?;
                Ok(ASTNode::new(IterStat::DoStat(ASTNode::new(DoStat::new(
                    stat, exp,
                )))))
            }

            // While '(' singleExpression ')' statement		# WhileStatement
            TokenKind::KeyWord(KeyWordKind::While) => {
                self.eat(TokenKind::KeyWord(KeyWordKind::While))?;

                self.eat(TokenKind::LeftParen)?;
                let exp = self.parse_exp()?;
                self.eat(TokenKind::RightParen)?;
                let stat = self.parse_stat()?;
                Ok(ASTNode::new(IterStat::WhileStat(ASTNode::new(
                    WhileStat::new(exp, stat),
                ))))
            }

            TokenKind::KeyWord(KeyWordKind::For) => {
                if let Some(for_stat) = self.try_to(Parser::parse_for_stat) {
                    return Ok(ASTNode::new(IterStat::ForStat(for_stat)));
                }

                if let Some(forin_stat) = self.try_to(Parser::parse_forin_stat) {
                    return Ok(ASTNode::new(IterStat::ForInStat(forin_stat)));
                }

                if let Some(forvar_stat) = self.try_to(Parser::parse_forvar_stat) {
                    return Ok(ASTNode::new(IterStat::ForVarStat(forvar_stat)));
                }

                Err(self.expect_error(
                    "For Statement",
                    "for statement or forin statement or forvar statement",
                ))
            }

            _ => unreachable!(),
        }
    }

    /*
    For '(' expressionSequence? SemiColon expressionSequence? SemiColon expressionSequence? ')' statement # ForStatement
    */
    fn parse_for_stat(&mut self) -> Result<ASTNode<ForStat>, ParserError> {
        let mut for_stat = ForStat::default();
        self.eat(TokenKind::KeyWord(KeyWordKind::For))?;
        self.eat(TokenKind::LeftParen)?;
        if !self.kind_is(TokenKind::SemiColon) {
            for_stat.set_init(self.parse_exp_seq()?);
        }
        self.eat(TokenKind::SemiColon)?;
        if !self.kind_is(TokenKind::SemiColon) {
            for_stat.set_cond(self.parse_exp()?);
        }
        self.eat(TokenKind::SemiColon)?;
        if !self.kind_is(TokenKind::SemiColon) {
            for_stat.set_action(self.parse_exp_seq()?);
        }
        self.eat(TokenKind::RightParen)?;
        for_stat.set_stat(self.parse_stat()?);
        Ok(ASTNode::new(for_stat))
    }

    /*
    For '(' identifier In expression ')' statement	# ForInStatement;
    */
    fn parse_forin_stat(&mut self) -> Result<ASTNode<ForInStat>, ParserError> {
        let var;
        let exp;
        let stat;
        self.eat(TokenKind::KeyWord(KeyWordKind::For))?;
        self.eat(TokenKind::LeftParen)?;
        var = ASTNode::new( Exp::Identifier(ASTNode::new( Identifier::new(&self.extact_identifier()?))));
        self.eat(TokenKind::KeyWord(KeyWordKind::In))?;
        exp = self.parse_exp()?;
        self.eat(TokenKind::RightParen)?;
        stat = self.parse_stat()?;
        Ok(ASTNode::new(ForInStat::new(var, exp, stat)))
    }

    /*
    For '(' varModifier variableDeclarationList SemiColon expressionSequence? SemiColon expressionSequence? ')' statement							# ForVarStatement
    )?;
    */
    fn parse_forvar_stat(&mut self) -> Result<ASTNode<ForVarStat>, ParserError> {
        let var_modifier;
        let var_decl_list;
        let mut cond = None;
        let mut action = None;
        let stat;

        self.eat(TokenKind::KeyWord(KeyWordKind::For))?;
        self.eat(TokenKind::LeftParen)?;
        match self.peek_kind() {
            TokenKind::KeyWord(KeyWordKind::Let) => {
                var_modifier = ASTNode::new(VarModifier::Let);
                self.eat(TokenKind::KeyWord(KeyWordKind::Let))?;
            }
            TokenKind::KeyWord(KeyWordKind::Var) => {
                var_modifier = ASTNode::new(VarModifier::Var);
                self.eat(TokenKind::KeyWord(KeyWordKind::Var))?;
            }
            TokenKind::KeyWord(KeyWordKind::Const) => {
                var_modifier = ASTNode::new(VarModifier::Const);
                self.eat(TokenKind::KeyWord(KeyWordKind::Const))?;
            }
            _ => return Err(self.expect_error("For Var Statement", "Let or Var or Const")),
        }

        var_decl_list = self.parse_var_decl_list()?;
        self.eat(TokenKind::SemiColon)?;
        if !self.kind_is(TokenKind::SemiColon) {
            cond = Some(self.parse_exp()?);
        }
        self.eat(TokenKind::SemiColon)?;
        if !self.kind_is(TokenKind::RightParen) {
            action = Some(self.parse_exp_seq()?);
        }

        self.eat(TokenKind::RightParen)?;

        stat = self.parse_stat()?;

        Ok(ASTNode::new(ForVarStat::new(
            var_modifier,
            var_decl_list,
            cond,
            action,
            stat,
        )))
    }

    /*
    variableDeclarationList: variableDeclaration (',' variableDeclaration)*;
    */
    fn parse_var_decl_list(&mut self) -> Result<ASTNode<VarDeclList>, ParserError> {
        let mut var_decl_list = VarDeclList::default();
        loop {
            var_decl_list.push_var_decl(self.parse_var_decl()?);
            if !self.kind_is(TokenKind::Comma) {
                break;
            }
            self.eat(TokenKind::Comma)?;
        }
        Ok(ASTNode::new(var_decl_list))
    }

    /*
    variableDeclaration: (Identifier | arrayLiteral | objectLiteral) typeAnnotation? ('=' singleExpression)?;
    */
    fn parse_var_decl(&mut self) -> Result<ASTNode<VarDecl>, ParserError> {
        match self.peek_kind() {
            TokenKind::Identifier => {
                let mut var_decl = VarDecl::new(&self.extact_identifier()?);
                if self.kind_is(TokenKind::Colon) {
                    self.eat(TokenKind::Colon)?;
                    var_decl.set_type_annotation(self.parse_type_annotation()?);
                }
                if self.kind_is(TokenKind::Assign) {
                    self.eat(TokenKind::Assign)?;
                    var_decl.set_initializer(self.parse_exp()?);
                }
                Ok(ASTNode::new(var_decl))
            }
            TokenKind::LeftBrace => Err(self.unsupported_error("arrayLiteral declare")),
            TokenKind::LeftBracket => Err(self.unsupported_error("objectLiteral declare")),
            _ => Err(self.expect_error("Var Decl", "Identifier or arrayLiteral or objectLiteral")),
        }
    }

    // fn parse_continue_stat(&mut self) -> Result<Option<ASTNode<ContinueStat>>, ParserError> {
    //     self.eat(TokenKind::KeyWord(KeyWordKind::Continue));
    //     // todo
    //     let eos = self.parse_eos()?;
    //     todo!()
    // }

    // fn parse_break_stat(&mut self) -> Result<Option<ASTNode<BreakStat>>, ParserError> {
    //     self.eat(TokenKind::KeyWord(KeyWordKind::Break));
    //     // todo
    //     let eos = self.parse_eos()?;
    //     todo!()
    // }

    /*
    Return (singleExpression)? eos;
    */
    fn parse_return_stat(&mut self) -> Result<ASTNode<ReturnStat>, ParserError> {
        let mut return_stat = ReturnStat::default();
        self.eat(TokenKind::KeyWord(KeyWordKind::Return))?;
        if !self.is_eos() {
            return_stat.set_exp_seq(self.parse_exp()?);
        }
        self.eat_eos()?;
        Ok(ASTNode::new(return_stat))
    }

    // fn parse_yield_stat(&mut self) -> Result<Option<ASTNode<YieldStat>>, ParserError> {
    //     self.eat(TokenKind::KeyWord(KeyWordKind::Yield));
    //     // todo
    //     let eos = self.parse_eos()?;
    //     todo!()
    // }

    // //    : With '(' expressionSequence ')' statement
    // fn parse_with_stat(&mut self) -> Result<Option<ASTNode<WithStat>>, ParserError> {
    //     self.eat(TokenKind::KeyWord(KeyWordKind::With));
    //     self.eat(TokenKind::LeftParen);
    //     let exp_seq = self.parse_exp_seq()?;
    //     self.eat(TokenKind::RightParen);
    //     let stat = self.parse_stat()?;
    //     todo!()
    // }

    // // Identifier ':' statement
    // fn parse_labelled_stat(&mut self) -> Result<Option<ASTNode<LabelledStat>>, ParserError> {
    //     self.eat(TokenKind::Identifier);
    //     self.eat(TokenKind::Colon);
    //     let stat = self.parse_stat()?;

    //     todo!()
    // }

    // // Switch '(' expressionSequence ')' caseBlock
    // fn parse_switch_stat(&mut self) -> Result<Option<ASTNode<SwitchStat>>, ParserError> {
    //     self.eat(TokenKind::KeyWord(KeyWordKind::Switch));
    //     self.eat(TokenKind::LeftParen);
    //     let exp_seq = self.parse_exp_seq()?;
    //     self.eat(TokenKind::RightParen);
    //     let cases = self.parse_case_block()?;
    //     todo!()
    // }

    // //  '{' caseClauses? (defaultClause caseClauses?)? '}'
    // fn parse_case_block(&mut self) -> Result<Option<ASTNode<CaseBlock>>, ParserError> {
    //     self.eat(TokenKind::LeftBracket);
    //     todo!();
    //     self.eat(TokenKind::RightBracket);
    // }

    // // caseClause+
    // fn parse_case_clause(&mut self) -> Result<Option<ASTNode<CaseClause>>, ParserError> {
    //     todo!()
    // }

    // // Case expressionSequence ':' statementList?
    // fn parse_case(&mut self) -> Result<Option<ASTNode<Case>>, ParserError> {
    //     self.eat(TokenKind::KeyWord(KeyWordKind::Case));
    //     let exp_seq = self.parse_exp_seq()?;
    //     self.eat(TokenKind::Colon);

    //     // wether statementList ?
    //     let stats = self.parse_stat_list()?;
    //     todo!()
    // }

    // // Throw {this.notLineTerminator()}? expressionSequence eos
    // fn parse_throw_stat(&mut self) -> Result<Option<ASTNode<ThrowStat>>, ParserError> {
    //     self.eat(TokenKind::KeyWord(KeyWordKind::Throw));
    //     let exp_seq = self.parse_exp_seq()?;
    //     let eos = self.parse_eos()?;
    //     todo!()
    // }

    // // Try block (catchProduction finallyProduction? | finallyProduction)
    // fn parse_try_stat(&mut self) -> Result<Option<ASTNode<TryStat>>, ParserError> {
    //     self.eat(TokenKind::KeyWord(KeyWordKind::Try));
    //     let block = self.parse_block()?;
    //     todo!()
    // }

    // fn parse_debugger_stat(&self) -> Result<Option<ASTNode<DebuggerStat>>, ParserError> {
    //     todo!()
    // }

    // functionDeclaration:
    // Function_ Identifier callSignature (
    //     ('{' functionBody '}')
    //     | SemiColon
    // );
    fn parse_func_declaration(&mut self) -> Result<ASTNode<FuncDecl>, ParserError> {
        let mut func_decl = FuncDecl::default();

        // 函数声明
        self.eat(TokenKind::KeyWord(KeyWordKind::Function))?;
        func_decl.set_func_name(&self.extact_identifier()?);

        func_decl.set_call_sig(self.parse_call_sig()?);
        if self.kind_is(TokenKind::SemiColon) {
            self.eat(TokenKind::SemiColon)?;
        } else {
            self.eat(TokenKind::LeftBracket)?;
            func_decl.set_func_body(self.parse_func_body()?);
            self.eat(TokenKind::RightBracket)?;
        }

        Ok(ASTNode::new(func_decl))
    }

    // functionExpressionDeclaration:
    // Function_ Identifier? '(' formalParameterList? ')' typeAnnotation? '{' functionBody '}';
    fn parse_func_exp_declaration(&mut self) -> Result<ASTNode<FuncExpDecl>, ParserError> {
        let mut func_exp_decl = FuncExpDecl::default();
        self.eat(TokenKind::KeyWord(KeyWordKind::Function))?;
        if self.kind_is(TokenKind::Identifier) {
            func_exp_decl.set_func_name(&self.extact_identifier()?);
        }
        self.eat(TokenKind::LeftParen)?;
        if !self.kind_is(TokenKind::RightParen) {
            func_exp_decl.set_formal_paras(self.parse_formal_parameters()?);
        }
        self.eat(TokenKind::RightParen)?;

        if self.kind_is(TokenKind::Colon) {
            func_exp_decl.set_type_annotation(self.parse_type_annotation()?);
        }

        self.eat(TokenKind::LeftBracket)?;
        func_exp_decl.set_func_body(self.parse_func_body()?);
        self.eat(TokenKind::RightBracket)?;

        Ok(ASTNode::new(func_exp_decl))
    }

    /*
    callSignature:
        typeParameters? '(' parameterList? ')' typeAnnotation?;
    */
    fn parse_call_sig(&mut self) -> Result<ASTNode<CallSig>, ParserError> {
        let mut call_sig = CallSig::default();
        if self.kind_is(TokenKind::LessThan) {
            call_sig.set_type_paras(self.parse_type_paras()?);
        }
        self.eat(TokenKind::LeftParen)?;
        match self.peek_kind() {
            TokenKind::Ellipsis
            | TokenKind::Identifier
            | TokenKind::LeftBracket
            | TokenKind::LeftBrace => call_sig.set_para_list(self.parse_para_list()?),
            _ => (), // nothing to do
        }
        self.eat(TokenKind::RightParen)?;
        if self.kind_is(TokenKind::Colon) {
            call_sig.set_type_annotation(self.parse_type_annotation()?);
        }

        Ok(ASTNode::new(call_sig))
    }

    /*
    constructSignature:
        'new' typeParameters? '(' parameterList? ')' typeAnnotation?;
    */
    fn parse_construct_sig(&mut self) -> Result<ASTNode<ConstructSig>, ParserError> {
        let mut construct_sig = ConstructSig::default();
        self.eat(TokenKind::KeyWord(KeyWordKind::New))?;
        if self.kind_is(TokenKind::LessThan) {
            construct_sig.set_type_paras(self.parse_type_paras()?);
        }

        self.eat(TokenKind::LeftParen)?;
        if !self.kind_is(TokenKind::RightParen) {
            construct_sig.set_para_list(self.parse_para_list()?)
        }

        self.eat(TokenKind::RightParen)?;

        if self.kind_is(TokenKind::Colon) {
            construct_sig.set_type_annotation(self.parse_type_annotation()?);
        }

        Ok(ASTNode::new(construct_sig))
    }

    /*
    propertySignature:
        ReadOnly? Identifier '?'? typeAnnotation?;
    */
    fn parse_property_sig(&mut self) -> Result<ASTNode<PropertySig>, ParserError> {
        let mut property_sig = PropertySig::default();
        if self.kind_is(TokenKind::KeyWord(KeyWordKind::ReadOnly)) {
            property_sig.set_readonly();
            self.eat(TokenKind::KeyWord(KeyWordKind::ReadOnly))?;
        }

        property_sig.set_property_name(&self.extact_identifier()?);

        if self.kind_is(TokenKind::QuestionMark) {
            property_sig.set_question_mark();
            self.eat(TokenKind::QuestionMark)?;
        }

        if self.kind_is(TokenKind::Colon) {
            property_sig.set_type_annotation(self.parse_type_annotation()?);
        }

        Ok(ASTNode::new(property_sig))
    }

    /*
    methodSignature: Identifier '?'? callSignature;
    */
    fn parse_method_sig(&mut self) -> Result<ASTNode<MethodSig>, ParserError> {
        let mut method_sig = MethodSig::default();
        method_sig.set_method_name(&self.extact_identifier()?);
        if self.kind_is(TokenKind::QuestionMark) {
            method_sig.set_question_mark();
            self.eat(TokenKind::QuestionMark)?;
        }

        method_sig.set_call_sig(self.parse_call_sig()?);

        Ok(ASTNode::new(method_sig))
    }

    /*
    functionBody
        : sourceElements?
        ;
    functionBody 左右必是被 { } 包围
    */
    fn parse_func_body(&mut self) -> Result<ASTNode<FuncBody>, ParserError> {
        let mut func_body = FuncBody::default();

        if self.kind_is(TokenKind::RightBracket) {
            return Ok(ASTNode::new(func_body));
        }
        func_body.set_func_body(self.parse_source_elements()?);
        Ok(ASTNode::new(func_body))
    }

    /*
    生成器函数声明
    Function_ '*' Identifier? '(' formalParameterList? ')' '{' functionBody '}'
     */
    fn parse_generator_func_declaration(&mut self) -> Result<ASTNode<GenFuncDecl>, ParserError> {
        self.eat(TokenKind::KeyWord(KeyWordKind::Function))?;
        self.eat(TokenKind::Multiply)?;
        if self.kind_is(TokenKind::Identifier) {
            todo!()
        }
        self.eat(TokenKind::LeftParen)?;
        if !self.kind_is(TokenKind::RightParen) {
            let formal_parameters = self.parse_formal_parameters()?;
        }
        self.eat(TokenKind::RightParen)?;

        self.eat(TokenKind::LeftBracket)?;
        let func_body = self.parse_func_body()?;
        self.eat(TokenKind::RightBracket)?;
        todo!()
    }

    /*
    formalParameterList:
        formalParameterArg (',' formalParameterArg)* (',' lastFormalParameterArg)?
        | lastFormalParameterArg;
    */
    fn parse_formal_parameters(&mut self) -> Result<ASTNode<FormalParas>, ParserError> {
        let mut formal_paras = FormalParas::default();

        if self.kind_is(TokenKind::Ellipsis) {
            self.eat(TokenKind::Ellipsis)?;
            formal_paras.set_last_para_arg(&self.extact_identifier()?);
        } else {
            loop {
                let formal_parameter_arg = self.parse_formal_parameter_arg()?;
                formal_paras.push_formal_para(formal_parameter_arg);
                match self.peek_kind() {
                    TokenKind::Comma => {
                        self.eat(TokenKind::Comma)?;
                        if self.kind_is(TokenKind::Ellipsis) {
                            self.eat(TokenKind::Ellipsis)?;
                            formal_paras.set_last_para_arg(&self.extact_identifier()?);
                            break;
                        }
                    }
                    _ => break,
                }
            }
        }

        Ok(ASTNode::new(formal_paras))
    }

    /*
    formalParameterArg:
        decorator? accessibilityModifier? Identifier '?'? typeAnnotation?;
    */
    fn parse_formal_parameter_arg(&mut self) -> Result<ASTNode<FormalPara>, ParserError> {
        let mut formal_para = FormalPara::default();
        if self.kind_is(TokenKind::At) {
            formal_para.set_decorator();
            self.eat(TokenKind::At)?;
        }

        match self.peek_kind() {
            TokenKind::KeyWord(KeyWordKind::Public) => {
                formal_para.set_access_modifier(KeyWordKind::Public);
                self.eat(TokenKind::KeyWord(KeyWordKind::Public))?;
            }
            TokenKind::KeyWord(KeyWordKind::Private) => {
                formal_para.set_access_modifier(KeyWordKind::Private);
                self.eat(TokenKind::KeyWord(KeyWordKind::Private))?;
            }
            TokenKind::KeyWord(KeyWordKind::Protected) => {
                formal_para.set_access_modifier(KeyWordKind::Protected);
                self.eat(TokenKind::KeyWord(KeyWordKind::Protected))?;
            }
            _ => (),
        }

        formal_para.set_identifier(&self.extact_identifier()?);

        if self.kind_is(TokenKind::QuestionMark) {
            formal_para.set_question_mark();
            self.eat(TokenKind::QuestionMark)?;
        }

        if self.kind_is(TokenKind::Colon) {
            formal_para.set_type_annotation(self.parse_type_annotation()?);
        }

        Ok(ASTNode::new(formal_para))
    }

    // expression (',' expression)*
    fn parse_exp_seq(&mut self) -> Result<ASTNode<ExpSeq>, ParserError> {
        let mut exp_seq = ExpSeq::default();
        loop {
            let exp = self.parse_exp()?;
            exp_seq.push_exp(exp);
            if !self.kind_is(TokenKind::Comma) {
                return Ok(ASTNode::new(exp_seq));
            }

            self.eat(TokenKind::Comma)?;
        }
    }

    /*
    typeParameters: '<' typeParameterList? '>';
    typeParameterList: typeParameter (',' typeParameter)*;
    */
    fn parse_type_paras(&mut self) -> Result<ASTNode<TypeParas>, ParserError> {
        Err(self.unsupported_error("type parameters"))
    }

    /*
    typeArguments: '<' typeArgumentList? '>';
    typeArgumentList: typeArgument (',' typeArgument)*;
    */
    fn parse_type_args(&mut self) -> Result<ASTNode<TypeArgs>, ParserError> {
        Err(self.unsupported_error("type arguments"))
    }

    /*
    parameterList:
        restParameter
        | parameter (',' parameter)* (',' restParameter)?;
    */
    fn parse_para_list(&mut self) -> Result<ASTNode<ParaList>, ParserError> {
        let mut para_list = ParaList::default();

        match self.peek_kind() {
            TokenKind::Ellipsis => {
                let rest_para = self.parse_rest_para()?;
                para_list.set_rest_para(rest_para);
                Ok(ASTNode::new(para_list))
            }
            TokenKind::Identifier | TokenKind::LeftBracket | TokenKind::LeftBrace => {
                loop {
                    let para = self.parse_para()?;
                    para_list.push_para(para);
                    match self.peek_kind() == TokenKind::Comma {
                        true => {
                            self.eat(TokenKind::Comma)?;
                            continue;
                        }
                        false => break,
                    }
                }

                if self.peek_kind() == TokenKind::Ellipsis {
                    para_list.set_rest_para(self.parse_rest_para()?);
                }

                Ok(ASTNode::new(para_list))
            }
            _ => Err(self.expect_error("ParaList", "Identifier or Ellipsis")),
        }
    }

    fn parse_type_annotation(&mut self) -> Result<ASTNode<TypeAnnotation>, ParserError> {
        self.eat(TokenKind::Colon)?;
        let type_ = self.parse_type()?;
        Ok(ASTNode::new(TypeAnnotation::new(type_)))
    }

    fn parse_rest_para(&self) -> Result<ASTNode<RestPara>, ParserError> {
        todo!()
    }

    /*
    parameter:
        decoratorList? accessibilityModifier? Identifier (
                '?' typeAnnotation?
                | typeAnnotation? initializer?
            )?
        ;
    */
    fn parse_para(&mut self) -> Result<ASTNode<Para>, ParserError> {
        if self.kind_is(TokenKind::LeftBrace) || self.kind_is(TokenKind::LeftBracket) {
            return Err(self.unsupported_error("bindingPattern"));
        }

        let mut para = Para::default();
        if self.kind_is(TokenKind::At) {
            para.set_decorators(self.parse_decorators()?);
        }

        match self.peek_kind() {
            TokenKind::KeyWord(KeyWordKind::Public)
            | TokenKind::KeyWord(KeyWordKind::Protected)
            | TokenKind::KeyWord(KeyWordKind::Private) => {
                para.set_access_modifier(self.parse_access_modifier()?);
            }
            _ => (),
        }

        para.set_para_name(&self.extact_identifier()?);

        if self.kind_is(TokenKind::QuestionMark) {
            para.set_question_mark();
            self.eat(TokenKind::QuestionMark)?;
            if self.kind_is(TokenKind::Colon) {
                para.set_type_annotation(self.parse_type_annotation()?);
            }
        } else {
            if self.kind_is(TokenKind::Colon) {
                para.set_type_annotation(self.parse_type_annotation()?);
            }

            if self.kind_is(TokenKind::Assign) {
                self.eat(TokenKind::Assign)?;
                para.set_initializer(self.parse_exp()?);
            }
        }

        Ok(ASTNode::new(para))
    }

    fn parse_type(&mut self) -> Result<ASTNode<Type>, ParserError> {
        if self.kind_is(TokenKind::LeftParen) {
            Ok(ASTNode::new(Type::FunctionType(self.parse_func_type()?)))
        } else {
            Ok(ASTNode::new(Type::PrimaryType(self.parse_primary_type()?)))
        }
    }

    /*
    primaryType:
        predefinedType								# PredefinedPrimType
        | typeReference								# ReferencePrimType
        | (predefinedType | typeReference) '[' ']'	# ArrayPrimType
        | '[' tupleElementTypes ']'					# TuplePrimType
        | objectType								# ObjectPrimType;
    */
    fn parse_primary_type(&mut self) -> Result<ASTNode<PrimaryType>, ParserError> {
        if self.kind_is(TokenKind::LeftBrace) {
            self.eat(TokenKind::LeftBrace)?;
            let tuple_type = PrimaryType::TupleType(self.parse_tuple_type()?);
            self.eat(TokenKind::RightBrace)?;
            return Ok(ASTNode::new(tuple_type));
        }

        if self.kind_is(TokenKind::LeftBracket) {
            return Ok(ASTNode::new(PrimaryType::ObjectType(
                self.parse_object_type()?,
            )));
        }

        match self.peek_kind() {
            TokenKind::KeyWord(KeyWordKind::Any) => {
                self.eat(TokenKind::KeyWord(KeyWordKind::Any))?;
                if self.kind_is(TokenKind::LeftBrace) {
                    self.eat(TokenKind::LeftBrace)?;
                    self.eat(TokenKind::RightBrace)?;
                    Ok(ASTNode::new(PrimaryType::ArrayPredefinedType(
                        ASTNode::new(ArrayPredefinedType::new(PredefinedType::Any)),
                    )))
                } else {
                    Ok(ASTNode::new(PrimaryType::PredefinedType(ASTNode::new(
                        PredefinedType::Any,
                    ))))
                }
            }

            TokenKind::KeyWord(KeyWordKind::Number) => {
                self.eat(TokenKind::KeyWord(KeyWordKind::Number))?;
                if self.kind_is(TokenKind::LeftBrace) {
                    self.eat(TokenKind::LeftBrace)?;
                    self.eat(TokenKind::RightBrace)?;
                    Ok(ASTNode::new(PrimaryType::ArrayPredefinedType(
                        ASTNode::new(ArrayPredefinedType::new(PredefinedType::Number)),
                    )))
                } else {
                    Ok(ASTNode::new(PrimaryType::PredefinedType(ASTNode::new(
                        PredefinedType::Number,
                    ))))
                }
            }

            TokenKind::KeyWord(KeyWordKind::Boolean) => {
                self.eat(TokenKind::KeyWord(KeyWordKind::Boolean))?;
                if self.kind_is(TokenKind::LeftBrace) {
                    self.eat(TokenKind::LeftBrace)?;
                    self.eat(TokenKind::RightBrace)?;
                    Ok(ASTNode::new(PrimaryType::ArrayPredefinedType(
                        ASTNode::new(ArrayPredefinedType::new(PredefinedType::Boolean)),
                    )))
                } else {
                    Ok(ASTNode::new(PrimaryType::PredefinedType(ASTNode::new(
                        PredefinedType::Boolean,
                    ))))
                }
            }

            TokenKind::KeyWord(KeyWordKind::String) => {
                self.eat(TokenKind::KeyWord(KeyWordKind::String))?;
                if self.kind_is(TokenKind::LeftBrace) {
                    self.eat(TokenKind::LeftBrace)?;
                    self.eat(TokenKind::RightBrace)?;
                    Ok(ASTNode::new(PrimaryType::ArrayPredefinedType(
                        ASTNode::new(ArrayPredefinedType::new(PredefinedType::String)),
                    )))
                } else {
                    Ok(ASTNode::new(PrimaryType::PredefinedType(ASTNode::new(
                        PredefinedType::String,
                    ))))
                }
            }

            TokenKind::KeyWord(KeyWordKind::Symbol) => {
                self.eat(TokenKind::KeyWord(KeyWordKind::Symbol))?;
                if self.kind_is(TokenKind::LeftBrace) {
                    self.eat(TokenKind::LeftBrace)?;
                    self.eat(TokenKind::RightBrace)?;
                    Ok(ASTNode::new(PrimaryType::ArrayPredefinedType(
                        ASTNode::new(ArrayPredefinedType::new(PredefinedType::Symbol)),
                    )))
                } else {
                    Ok(ASTNode::new(PrimaryType::PredefinedType(ASTNode::new(
                        PredefinedType::Symbol,
                    ))))
                }
            }

            TokenKind::KeyWord(KeyWordKind::Void) => {
                self.eat(TokenKind::KeyWord(KeyWordKind::Void))?;
                if self.kind_is(TokenKind::LeftBrace) {
                    self.eat(TokenKind::LeftBrace)?;
                    self.eat(TokenKind::RightBrace)?;
                    Ok(ASTNode::new(PrimaryType::ArrayPredefinedType(
                        ASTNode::new(ArrayPredefinedType::new(PredefinedType::Void)),
                    )))
                } else {
                    Ok(ASTNode::new(PrimaryType::PredefinedType(ASTNode::new(
                        PredefinedType::Void,
                    ))))
                }
            }

            TokenKind::Identifier => {
                let mut type_ref = TypeRef::default();
                type_ref.set_type_name(&self.extact_identifier()?);
                if self.kind_is(TokenKind::LeftBrace) {
                    self.eat(TokenKind::LeftBrace)?;
                    self.eat(TokenKind::RightBrace)?;
                    Ok(ASTNode::new(PrimaryType::ArrayTypeRef(ASTNode::new(
                        ArrayTypeRef::new(type_ref),
                    ))))
                } else {
                    Ok(ASTNode::new(PrimaryType::TypeRef(ASTNode::new(type_ref))))
                }
            }

            _ => Err(self.expect_error(
                "Parse Primary Type",
                "Predefined Type or Tuple Type or Type Ref",
            )),
        }
    }

    fn parse_tuple_type(&mut self) -> Result<ASTNode<TupleElementTypes>, ParserError> {
        todo!()
    }

    /*
    functionType: '(' parameterList? ')' '=>' type_;
    */
    fn parse_func_type(&mut self) -> Result<ASTNode<FunctionType>, ParserError> {
        let type_;
        let mut para_list = None;
        self.eat(TokenKind::LeftParen)?;
        if !self.kind_is(TokenKind::RightParen) {
            para_list = Some(self.parse_para_list()?);
        }
        self.eat(TokenKind::RightParen)?;
        self.eat(TokenKind::ARROW)?;
        type_ = self.parse_type()?;

        Ok(ASTNode::new(FunctionType::new(para_list, type_)))
    }

    fn parse_decorators(&mut self) -> Result<ASTNode<Decorators>, ParserError> {
        Err(self.unsupported_error("decorators"))
    }

    fn parse_access_modifier(&mut self) -> Result<ASTNode<AccessModifier>, ParserError> {
        match self.peek_kind() {
            TokenKind::KeyWord(KeyWordKind::Public) => {
                self.eat(TokenKind::KeyWord(KeyWordKind::Public))?;
                Ok(ASTNode::new(AccessModifier::Public))
            }
            TokenKind::KeyWord(KeyWordKind::Private) => {
                self.eat(TokenKind::KeyWord(KeyWordKind::Private))?;
                Ok(ASTNode::new(AccessModifier::Private))
            }
            TokenKind::KeyWord(KeyWordKind::Protected) => {
                self.eat(TokenKind::KeyWord(KeyWordKind::Protected))?;
                Ok(ASTNode::new(AccessModifier::Protected))
            }
            _ => Err(self.expect_error("Access Modifier", "public or protected or private")),
        }
    }

    /*
    interfaceDeclaration:
    Export? Declare? Interface Identifier typeParameters? (
        Extends typeReference (',' typeReference)*
    )? objectType SemiColon?;
    */
    fn parse_interface_decl(&mut self) -> Result<ASTNode<InterfaceDecl>, ParserError> {
        let mut interface_decl = InterfaceDecl::default();
        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Export)) {
            interface_decl.set_export();
            self.eat(TokenKind::KeyWord(KeyWordKind::Export))?;
        }
        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Declare)) {
            interface_decl.set_declare();
            self.eat(TokenKind::KeyWord(KeyWordKind::Declare))?;
        }
        self.eat(TokenKind::KeyWord(KeyWordKind::Interface))?;

        interface_decl.set_identifier(&self.extact_identifier()?);
        if self.kind_is(TokenKind::LessThan) {
            interface_decl.set_type_paras(self.parse_type_paras()?);
        }

        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Extends)) {
            self.eat(TokenKind::KeyWord(KeyWordKind::Extends))?;
            loop {
                interface_decl.push_extends(ASTNode::new(Extends::new(self.parse_type_ref()?)));
                if !self.kind_is(TokenKind::Comma) {
                    break;
                }
                self.eat(TokenKind::Comma)?;
            }
        }

        interface_decl.set_object_type(self.parse_object_type()?);
        if self.kind_is(TokenKind::SemiColon) {
            self.eat(TokenKind::SemiColon)?;
        }

        Ok(ASTNode::new(interface_decl))
    }

    /*
    objectType:
        '{' (
            typeMember ((SemiColon | ',') typeMember)* (
                SemiColon
                | ','
            )?
        )? '}';
    */
    fn parse_object_type(&mut self) -> Result<ASTNode<ObjectType>, ParserError> {
        let mut object_type = ObjectType::default();
        self.eat(TokenKind::LeftBracket)?;
        loop {
            let type_member = self.parse_type_member()?;
            object_type.push_type_member(type_member);
            match self.peek_kind() {
                TokenKind::Comma => {
                    self.eat(TokenKind::Comma)?;
                }
                TokenKind::SemiColon => {
                    self.eat(TokenKind::SemiColon)?;
                }
                _ => {
                    if !self.kind_is(TokenKind::RightBracket) && !self.is_new_line() {
                        return Err(self.expect_error("Object Type", ", or ; or }"));
                    }
                }
            }

            if self.kind_is(TokenKind::RightBracket) {
                break;
            }
        }
        self.eat(TokenKind::RightBracket)?;
        Ok(ASTNode::new(object_type))
    }

    /*
    typeMember:
        propertySignature
        | callSignature
        | constructSignature
        | indexSignature
        | methodSignature;
        */
    fn parse_type_member(&mut self) -> Result<ASTNode<TypeMember>, ParserError> {
        match self.peek_kind() {
            TokenKind::LeftParen | TokenKind::LessThan => {
                Ok(ASTNode::new(TypeMember::CallSig(self.parse_call_sig()?)))
            }
            TokenKind::KeyWord(KeyWordKind::New) => {
                Ok(ASTNode::new(TypeMember::ConstructSig(self.parse_construct_sig()?)))
            }
            TokenKind::LeftBrace => {
                Ok(ASTNode::new(TypeMember::IndexSig(self.parse_index_sig()?)))
            }
            TokenKind::KeyWord(KeyWordKind::ReadOnly) => {
                Ok(ASTNode::new(TypeMember::PropertySig(self.parse_property_sig()?)))
            }
            TokenKind::Identifier => {
                // attention: do not exchange the order of this if below
                // because property_sig can be the prefix of the method_sig
                if let Some(method_sig) = self.try_to(Parser::parse_method_sig) {
                    return Ok(ASTNode::new(TypeMember::MethodSig(method_sig)));
                }

                if let Some(property_sig) = self.try_to(Parser::parse_property_sig) {
                    return Ok(ASTNode::new(TypeMember::PropertySig(property_sig)));
                }

                Err(self.expect_error("typeMember", "propertySignature or methodSignature"))
            }
            _ => {
                Err(self.expect_error("typeMember", "propertySignature or callSignature or constructSignature or indexSignature or methodSignature"))
            }
        }
    }

    fn parse_namespace_decl(&mut self) -> Result<ASTNode<NamespaceDecl>, ParserError> {
        todo!()
    }
}
