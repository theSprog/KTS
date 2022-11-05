pub mod error;

use crate::ast::ast_node::call_sig;
use crate::ast::ast_node::call_sig::CallSig;
use crate::ast::ast_node::eos::EOS;
use crate::ast::ast_node::program::Program;
use crate::ast::visulize::Visualizable;
use crate::ast::ASTNode;
use lazy_static::lazy_static;

use crate::ast::ast_node::block::*;
use crate::ast::ast_node::body::*;
use crate::ast::ast_node::case::*;
use crate::ast::ast_node::clause::*;
use crate::ast::ast_node::decl::*;
use crate::ast::ast_node::exp::*;
use crate::ast::ast_node::list::*;
use crate::ast::ast_node::parameter::*;
use crate::ast::ast_node::source_elements::*;
use crate::ast::ast_node::stat::*;

use crate::ast::ast_node::unknown::Unknown;
use crate::compiler_internal_error;
use crate::lexer::token::Token;
use crate::lexer::token_kind::{KeyWordKind, TokenKind};
use crate::{ast::AST, error::TSError};

use self::error::ParserError;

pub(crate) struct Parser {
    tokens: Vec<Token>,
    index: usize,
}
impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        AST::reset();
        Self { tokens, index: 0 }
    }

    pub(crate) fn show_tokens(&mut self) {
        for token in &self.tokens {
            println!("{}", token);
        }
    }

    // general error report
    fn report_error(&mut self, msg: &str) -> ParserError {
        let cur = self.peek().unwrap();
        ParserError::new(format!("Line[{}]: {}", cur.peek_line(), msg))
    }

    // expect token error
    fn expect_error(&mut self, stat: &str, expects: &str) -> ParserError {
        let cur = self.peek().unwrap();
        self.report_error(&format!(
            "{}: Expect [{}] but got token {}({})",
            stat,
            expects,
            cur.peek_value(),
            cur.peek_kind()
        ))
    }

    fn eat(&mut self, kind: TokenKind) -> Result<(), ParserError> {
        if let Some(token) = self.peek() {
            match token.peek_kind() == kind {
                true => {
                    self.index += 1;
                    Ok(())
                }
                false => Err(self.expect_error("Eat", &kind.to_string())),
            }
        } else {
            compiler_internal_error!("Can not eat token because there is no token");
        }
    }

    fn eat_eos(&mut self) -> Result<(), ParserError> {
        // 用分号可以  xxx; yyy
        if self.peek_kind() == TokenKind::SemiColon {
            self.eat(TokenKind::SemiColon)?;
            return Ok(());
        }

        if self.peek_kind() == TokenKind::EOF {
            return Ok(());
        }

        if let (Some(current), Some(next)) =
            (self.tokens.get(self.index), self.tokens.get(self.index - 1))
        {
            // 换行也允许  xxx \n yyy
            if current.peek_line() > next.peek_line() {
                return Ok(());
            } else {
                // 不用分号又不换行 xxx yyy 这种形式不允许
                return Err(self.expect_error("EOS", "; or newline"));
            }
        }

        unreachable!()
    }

    /*
    尝试函数，选择一个分支进行尝试，成功则返回，出错则回溯
    注意，成功也包括解析出 None
    事实上，解析出 None 是合法的，只有解析出 Err 才说明分支选择错误

    没事别用 try_to, 用 try_to 必须是在不确定分支选择的时候
    try_to 是有限个前瞻无法解决的时候使用，即处理 LL(*) 时采用。
    凡是有限个前瞻可以解决的，不适用 try_to，否则你可能会得到一个毫无意义的报错信息
    有 try_to 的地方必须有所有候选分支都不匹配的 Err
    */
    fn try_to<T: Visualizable>(
        &mut self,
        func: &dyn Fn(&mut Parser) -> Result<Option<ASTNode<T>>, ParserError>,
    ) -> Option<ASTNode<T>> {
        let current = self.index;
        match func(self) {
            Ok(stat) => stat,
            Err(_) => {
                self.index = current;
                None
            }
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    fn peek_kind(&self) -> TokenKind {
        match self.peek() {
            Some(token) => token.peek_kind(),
            None => compiler_internal_error!("Why it can be here?"),
        }
    }

    fn look_ahead(&self) -> Option<TokenKind> {
        match self.tokens.get(self.index + 1) {
            Some(token) => Some(token.peek_kind()),
            None => None,
        }
    }

    // fn look_ahead2(&self) -> Option<TokenKind> {
    //     match self.tokens.get(self.index + 2) {
    //         Some(token) => Some(token.peek_kind()),
    //         None => None,
    //     }
    // }

    // fn look_ahead3(&self) -> Option<TokenKind> {
    //     match self.tokens.get(self.index + 3) {
    //         Some(token) => Some(token.peek_kind()),
    //         None => None,
    //     }
    // }

    fn kind_is(&self, kind: TokenKind) -> bool {
        match self.peek() {
            Some(token) => token.kind_is(kind),
            None => false,
        }
    }

    pub(crate) fn parse(&mut self) -> Result<AST, TSError> {
        Ok(AST::new(self.parse_program()?))
    }

    fn parse_program(&mut self) -> Result<ASTNode<Program>, ParserError> {
        let mut programe = Program::new();

        match self.kind_is(TokenKind::EOF) {
            true => Ok(ASTNode::new(programe)),
            false => {
                if let Some(source_elements) = self.parse_source_elements()? {
                    programe.set_source_elements(source_elements);
                }
                Ok(ASTNode::new(programe))
            }
        }
    }

    // sourceElements: sourceElement+;
    fn parse_source_elements(&mut self) -> Result<Option<ASTNode<SourceElements>>, ParserError> {
        let mut source_elements = SourceElements::new();

        while let Some(source_element) = self.parse_source_element()? {
            source_elements.push_source_element(source_element);

            // sourceElement 只可能有两个 follow: { EOF, }(RightBracket) }
            if self.kind_is(TokenKind::EOF) || self.kind_is(TokenKind::RightBracket) {
                break;
            }
        }

        match source_elements.is_empty() {
            true => Ok(None), // if it is empty
            false => Ok(Some(ASTNode::new(source_elements))),
        }
    }

    // sourceElement: statement;
    fn parse_source_element(&mut self) -> Result<Option<ASTNode<SourceElement>>, ParserError> {
        let mut source_element = SourceElement::new();

        match self.parse_stat()? {
            Some(stat) => source_element.set_stat(stat),
            None => return Ok(None),
        }

        Ok(Some(ASTNode::new(source_element)))
    }

    fn parse_stat(&mut self) -> Result<Option<ASTNode<Stat>>, ParserError> {
        match self.peek_kind() {
            TokenKind::LeftBracket => match self.parse_block()? {
                Some(block) => Ok(Some(ASTNode::new(Stat::Block(block)))),
                None => Ok(None),
            },
            TokenKind::KeyWord(KeyWordKind::Import) => match self.parse_import_stat()? {
                Some(import_stat) => Ok(Some(ASTNode::new(Stat::ImportStat(import_stat)))),
                None => Ok(None),
            },
            TokenKind::KeyWord(KeyWordKind::Export) => match self.parse_export_stat()? {
                Some(export_stat) => Ok(Some(ASTNode::new(Stat::ExportStat(export_stat)))),
                None => Ok(None),
            },
            TokenKind::SemiColon => match self.parse_empty_stat()? {
                Some(empty_stat) => Ok(Some(ASTNode::new(Stat::EmptyStat(empty_stat)))),
                None => Ok(None),
            },

            // // abstract class or abstract ?
            // TokenKind::KeyWord(KeyWordKind::Abstract) => match self.lookAhead() {
            //     Some(TokenKind::KeyWord(KeyWordKind::Class)) => {
            //         Ok(Some(self.parse_class_declaration()?))
            //     }

            //     _ => Ok(Some(self.parse_abstract_declaration()?)),
            // },

            // TokenKind::KeyWord(KeyWordKind::Class) => Ok(Some(self.parse_abstract_declaration()?)),

            // TokenKind::KeyWord(KeyWordKind::If) => Ok(Some(self.parse_if_stat()?)),

            // // do|while|for -> iteration stat
            // TokenKind::KeyWord(KeyWordKind::Do)
            // | TokenKind::KeyWord(KeyWordKind::While)
            // | TokenKind::KeyWord(KeyWordKind::For) => Ok(Some(self.parse_iteration_stat()?)),

            // TokenKind::KeyWord(KeyWordKind::Continue) => Ok(Some(self.parse_continue_stat()?)),
            // TokenKind::KeyWord(KeyWordKind::Break) => Ok(Some(self.parse_break_stat()?)),
            // TokenKind::KeyWord(KeyWordKind::Return) => Ok(Some(self.parse_return_stat()?)),
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
            TokenKind::KeyWord(KeyWordKind::Function_) => match self.look_ahead() {
                Some(TokenKind::Identifier) => match self.parse_func_declaration()? {
                    Some(func_decl) => Ok(Some(ASTNode::new(Stat::FuncDecl(func_decl)))),
                    None => Ok(None),
                },
                Some(TokenKind::LeftParen) => match self.parse_func_exp_declaration()? {
                    Some(func_exp_decl) => Ok(Some(ASTNode::new(Stat::FuncExpDecl(func_exp_decl)))),
                    None => Ok(None),
                },
                Some(TokenKind::Multiply) => match self.parse_generator_func_declaration()? {
                    Some(gen_func_decl) => Ok(Some(ASTNode::new(Stat::GenFuncDecl(gen_func_decl)))),
                    None => Ok(None),
                },
                _ => Err(self.expect_error("Func", "Identifier, ( or *")),
            },

            // todo how to deal with arrow functions
            // todo how to deal with variable statement
            // todo how to deal with type aliases
            // todo how to deal with enum declarations
            // todo how to deal with exp declarations
            _ => {
                Err(self.report_error(&format!("Stat: Unexpected Token {}", self.peek().unwrap())))
            }
        }
    }

    fn parse_block(&mut self) -> Result<Option<ASTNode<Block>>, ParserError> {
        let mut block = Block::new();
        self.eat(TokenKind::LeftBracket)?;
        loop {
            match self.try_to(&Parser::parse_stat) {
                Some(stat) => block.push(stat),
                None => break,
            }
        }
        self.eat(TokenKind::RightBracket)?;
        Ok(Some(ASTNode::new(block)))
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
    fn parse_import_stat(&mut self) -> Result<Option<ASTNode<ImportStat>>, ParserError> {
        let mut import_stat = ImportStat::default();
        self.eat(TokenKind::KeyWord(KeyWordKind::Import))?;
        if let Some(from_block) = self.parse_from_block()? {
            import_stat.set_from_block(from_block);
        }
        Ok(Some(ASTNode::new(import_stat)))
    }

    fn parse_from_block(&mut self) -> Result<Option<ASTNode<FromBlock>>, ParserError> {
        let mut from_block = FromBlock::default();
        match self.peek_kind() {
            TokenKind::Multiply => {
                from_block.set_all();
                self.eat(TokenKind::Multiply)?;
                if self.kind_is(TokenKind::KeyWord(KeyWordKind::As)) {
                    self.eat(TokenKind::KeyWord(KeyWordKind::As))?;
                    match self.peek_kind() {
                        TokenKind::Identifier => {
                            from_block.set_all_alias(self.peek().unwrap().peek_value().as_str());
                            self.eat(TokenKind::Identifier)?;
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
                    let import = self.peek().unwrap().peek_value();
                    from_block.set_imported(import);
                    self.eat(TokenKind::Identifier)?;

                    if self.kind_is(TokenKind::Comma) {
                        self.eat(TokenKind::Comma)?;
                    }
                }
                // if it be "{a, b as c, ...}"
                if self.kind_is(TokenKind::LeftBracket) {
                    self.eat(TokenKind::LeftBracket)?;
                    while self.kind_is(TokenKind::Identifier) {
                        let imported = self.peek().unwrap().peek_value().clone();

                        self.eat(TokenKind::Identifier)?;
                        match self.kind_is(TokenKind::KeyWord(KeyWordKind::As)) {
                            true => {
                                self.eat(TokenKind::KeyWord(KeyWordKind::As))?;
                                match self.peek_kind() {
                                    TokenKind::Identifier => {
                                        let mut alias =
                                            Some(self.peek().unwrap().peek_value().as_str());
                                        from_block.push_imported_alias(&imported, alias);
                                        self.eat(TokenKind::Identifier)?;
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
        Ok(Some(ASTNode::new(from_block)))
    }

    /*
        exportStatement: Export Default? (fromBlock | statement);
    */
    fn parse_export_stat(&mut self) -> Result<Option<ASTNode<ExportStat>>, ParserError> {
        let mut export_stat = ExportStat::default();

        self.eat(TokenKind::KeyWord(KeyWordKind::Export));
        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Default)) {
            export_stat.set_default();
            self.eat(TokenKind::KeyWord(KeyWordKind::Default));
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
        if let Some(from_block) = self.try_to(&Parser::parse_from_block) {
            export_stat.set_from_block(from_block);
            return Ok(Some(ASTNode::new(export_stat)));
        }

        // 如果不是 from block, 那么说明一定是 stat
        // 从而保证进入这里面的 stat 一定不是 export 开头
        if let Some(stat) = self.parse_stat()? {
            export_stat.set_stat(stat);
            self.eat_eos();
            return Ok(Some(ASTNode::new(export_stat)));
        }

        // 两个都不是, 出错
        Err(self.report_error("Expect [FromBlock] or [Statment] but there is no such match"))
    }

    fn parse_empty_stat(&mut self) -> Result<Option<ASTNode<EmptyStat>>, ParserError> {
        self.eat(TokenKind::SemiColon);
        Ok(Some(ASTNode::new(EmptyStat::new())))
    }

    // fn parse_class_declaration(&mut self) -> Result<Option<ASTNode<ClassDecl>>, ParserError> {
    //     if self.kind_is(TokenKind::KeyWord(KeyWordKind::Abstract)) {
    //         self.eat(TokenKind::KeyWord(KeyWordKind::Abstract));
    //     }
    //     self.eat(TokenKind::KeyWord(KeyWordKind::Class));
    //     self.eat(TokenKind::Identifier);
    //     todo!()
    // }

    // fn parse_abstract_declaration(&mut self) -> Result<Option<ASTNode<AbsDecl>>, ParserError> {
    //     self.eat(TokenKind::KeyWord(KeyWordKind::Abstract));
    //     todo!()
    // }

    // fn parse_if_stat(&mut self) -> Result<Option<ASTNode<IfStat>>, ParserError> {
    //     self.eat(TokenKind::KeyWord(KeyWordKind::If));
    //     self.eat(TokenKind::LeftParen);
    //     let exp_seq = self.parse_exp_seq()?;
    //     self.eat(TokenKind::RightParen);
    //     let stat = self.parse_stat()?;
    //     if self.kind_is(TokenKind::KeyWord(KeyWordKind::Else)) {
    //         self.eat(TokenKind::KeyWord(KeyWordKind::Else));
    //         let else_stat = self.parse_stat()?;
    //     }
    //     todo!()
    // }

    // fn parse_iteration_stat(&mut self) -> Result<Option<ASTNode<IterStat>>, ParserError> {
    //     match self.peek_kind() {
    //         TokenKind::KeyWord(KeyWordKind::Do) => {
    //             self.eat(TokenKind::KeyWord(KeyWordKind::Do));
    //             let stat = self.parse_stat()?;
    //             self.eat(TokenKind::KeyWord(KeyWordKind::While));
    //             self.eat(TokenKind::LeftParen);
    //             let exp_seq = self.parse_exp_seq()?;
    //             self.eat(TokenKind::RightParen);
    //             let eos = self.parse_eos()?;
    //             todo!()
    //         }

    //         TokenKind::KeyWord(KeyWordKind::While) => {
    //             self.eat(TokenKind::KeyWord(KeyWordKind::While));

    //             self.eat(TokenKind::LeftParen);
    //             let exp_seq = self.parse_exp_seq()?;
    //             self.eat(TokenKind::RightParen);
    //             let stat = self.parse_stat()?;

    //             todo!()
    //         }

    //         TokenKind::KeyWord(KeyWordKind::For) => {
    //             self.eat(TokenKind::KeyWord(KeyWordKind::For));
    //             self.eat(TokenKind::LeftParen);
    //             todo!()
    //         }

    //         _ => compiler_internal_error!("Why is can be here?"),
    //     }
    // }

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

    // fn parse_return_stat(&mut self) -> Result<Option<ASTNode<ReturnStat>>, ParserError> {
    //     self.eat(TokenKind::KeyWord(KeyWordKind::Return));
    //     // todo
    //     let eos = self.parse_eos()?;
    //     todo!()
    // }

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
    // 需要区分到底是表达式声明还是函数声明
    fn parse_func_declaration(&mut self) -> Result<Option<ASTNode<FuncDecl>>, ParserError> {
        let mut func_decl = FuncDecl::new();

        // 函数声明
        self.eat(TokenKind::KeyWord(KeyWordKind::Function_))?;
        let func_name = self.peek().unwrap().peek_value();
        func_decl.set_func_name(func_name);
        self.eat(TokenKind::Identifier)?;

        match self.parse_call_signature()? {
            Some(call_sig) => func_decl.set_call_sig(call_sig),
            None => {
                return Err(self.report_error("FuncDecl: Expect Call Signature, Unexpected token"))
            }
        }

        if self.kind_is(TokenKind::SemiColon) {
            self.eat(TokenKind::SemiColon)?;
        } else {
            self.eat(TokenKind::LeftBracket)?;
            match self.parse_func_body()? {
                Some(func_body) => func_decl.set_func_body(func_body),
                None => {
                    return Err(
                        self.report_error("FuncDecl: Expect Function Body, Unexpected token")
                    )
                }
            }
            self.eat(TokenKind::RightBracket)?;
        }

        Ok(Some(ASTNode::new(func_decl)))
    }

    // functionExpressionDeclaration:
    // Function_ '(' formalParameterList? ')' typeAnnotation? '{' functionBody '}';
    fn parse_func_exp_declaration(&mut self) -> Result<Option<ASTNode<FuncExpDecl>>, ParserError> {
        todo!()
    }

    /*
    callSignature:
        typeParameters? '(' parameterList? ')' typeAnnotation?;
    */
    fn parse_call_signature(&mut self) -> Result<Option<ASTNode<CallSig>>, ParserError> {
        let mut call_sig = CallSig::new();
        if self.kind_is(TokenKind::LessThan) {
            match self.parse_type_paras()? {
                Some(type_paras) => call_sig.set_type_paras(type_paras),
                None => return Err(self.report_error("CallSig: Unexpected token")),
            }
        }
        self.eat(TokenKind::LeftParen)?;
        match self.peek_kind() {
            TokenKind::Ellipsis
            | TokenKind::Identifier
            | TokenKind::LeftBracket
            | TokenKind::LeftBrace => match self.parse_para_list()? {
                Some(para_list) => call_sig.set_para_list(para_list),
                None => return Err(self.report_error("CallSig: Unexpected token")),
            },
            _ => (), // nothing to do
        }
        self.eat(TokenKind::RightParen)?;
        if self.kind_is(TokenKind::Colon) {
            self.eat(TokenKind::Colon)?;
            match self.parse_type_annotation()? {
                Some(type_annotation) => call_sig.set_type_annotation(type_annotation),
                None => return Err(self.report_error("CallSig: Unexpected token")),
            }
        }

        Ok(Some(ASTNode::new(call_sig)))
    }

    // sourceElements?
    fn parse_func_body(&mut self) -> Result<Option<ASTNode<FuncBody>>, ParserError> {
        // try to
        let source_elements = self.parse_source_elements()?;
        todo!()
    }

    // Function_ '*' Identifier? '(' formalParameterList? ')' '{' functionBody '}'
    fn parse_generator_func_declaration(
        &mut self,
    ) -> Result<Option<ASTNode<GenFuncDecl>>, ParserError> {
        self.eat(TokenKind::KeyWord(KeyWordKind::Function_))?;
        self.eat(TokenKind::Multiply)?;
        if self.kind_is(TokenKind::Identifier) {
            todo!()
        }
        self.eat(TokenKind::LeftParen)?;
        let formal_parameters = self.parse_formal_parameters()?;
        self.eat(TokenKind::RightParen)?;

        self.eat(TokenKind::LeftBracket)?;
        let func_body = self.parse_func_body()?;
        self.eat(TokenKind::RightBracket)?;
        todo!()
    }

    // /*
    // formalParameterArg (',' formalParameterArg)* (',' lastFormalParameterArg)?
    // | lastFormalParameterArg
    // | arrayLiteral                              // ECMAScript 6: Parameter Context Matching
    // | objectLiteral (':' formalParameterList)?  // ECMAScript 6: Parameter Context Matching
    // */
    fn parse_formal_parameters(&mut self) -> Result<Option<ASTNode<FormalParas>>, ParserError> {
        todo!()
    }

    // // singleExpression (',' singleExpression)*
    // fn parse_exp_seq(&mut self) -> Result<Option<ASTNode<ExpSeq>>, ParserError> {
    //     let single_exp = self.parse_single_exp()?;
    //     while self.kind_is(TokenKind::Comma) {
    //         self.eat(TokenKind::Comma);
    //         let single_exp_other = self.parse_single_exp()?;
    //     }
    //     todo!()
    // }

    fn parse_single_exp(&self) -> Result<Option<ASTNode<Exp>>, ParserError> {
        // TODO: this is first to deal with
        todo!()
    }

    fn parse_type_paras(&self) -> Result<Option<ASTNode<TypeParas>>, ParserError> {
        // todo
        todo!()
    }

    fn parse_para_list(&mut self) -> Result<Option<ASTNode<ParaList>>, ParserError> {
        match self.peek_kind() {
            TokenKind::Ellipsis => match self.parse_rest_para()? {
                Some(rest_para) => {
                    let mut para_list = ParaList::new();
                    para_list.set_rest_para(rest_para);
                    Ok(Some(ASTNode::new(para_list)))
                }
                None => Ok(None),
            },
            TokenKind::Identifier | TokenKind::LeftBracket | TokenKind::LeftBrace => {
                let mut para_list = ParaList::new();
                while let Some(para) = self.parse_para()? {
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
                    let rest_para = match self.parse_rest_para()? {
                        Some(rest_para) => rest_para,
                        None => return Err(self.report_error("")),
                    };
                    para_list.set_rest_para(rest_para);
                }

                Ok(Some(ASTNode::new(para_list)))
            }
            _ => Err(self.report_error("ParaList: Unexpected Token")),
        }
    }

    fn parse_type_annotation(&self) -> Result<Option<ASTNode<TypeAnnotation>>, ParserError> {
        todo!()
    }

    fn parse_rest_para(&self) -> Result<Option<ASTNode<RestPara>>, ParserError> {
        todo!()
    }

    fn parse_para(&mut self) -> Result<Option<ASTNode<Para>>, ParserError> {
        match self.peek_kind() {
            TokenKind::Identifier => {
                let mut para = Para::new();
                let para_name = self.peek().unwrap().peek_value();
                para.set_para_name(para_name);
                self.eat(TokenKind::Identifier)?;
                Ok(Some(ASTNode::new(para)))
            }
            TokenKind::LeftBracket => {
                // 太复杂，不考虑
                todo!()
            }
            TokenKind::LeftBrace => {
                // 太复杂，不考虑
                todo!()
            }
            _ => Err(self.expect_error("Para", "Identifier")),
        }
    }

    // fn parse_eos(&mut self) -> Result<Option<ASTNode<EOS>>, ParserError> {
    //     if self.kind_is(TokenKind::SemiColon) {
    //         self.eat(TokenKind::SemiColon);
    //     } else if self.kind_is(TokenKind::EOF) {
    //         self.eat(TokenKind::EOF);
    //     }

    //     todo!()
    // }
}
