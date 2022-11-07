pub mod error;

use crate::ast::ast_node::call_sig;
use crate::ast::ast_node::call_sig::CallSig;

use crate::ast::ast_node::decorator::Decorators;
use crate::ast::ast_node::eos::EOS;
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
use crate::ast::ast_node::source_element::*;
use crate::ast::ast_node::stat::*;
use crate::ast::ast_node::type_ref::*;

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

    fn unsupported_error(&mut self, unsupported: &str) -> ParserError {
        let cur = self.peek().unwrap();
        self.report_error(&format!("Sorry, but now {} is not supported", unsupported))
    }

    fn eat(&mut self, kind: TokenKind) -> Result<(), ParserError> {
        if let Some(token) = self.peek() {
            match token.peek_kind() == kind {
                true => {
                    self.index += 1;
                    Ok(())
                }
                false => match kind {
                    TokenKind::SemiColon => Err(self.report_error(&format!(
                        "you might forgot ';' in the end of line[{}]",
                        self.tokens.get(self.index - 1).unwrap().peek_line()
                    ))),

                    _ => Err(self.expect_error("Eat", &kind.to_string())),
                },
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
        func: &dyn Fn(&mut Parser) -> Result<ASTNode<T>, ParserError>,
    ) -> Option<ASTNode<T>> {
        let current = self.index;
        match func(self) {
            Ok(stat) => Some(stat),
            Err(_) => {
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
                self.eat(TokenKind::Identifier);
                self.tokens
                    .get(self.index - 1)
                    .unwrap()
                    .peek_value()
                    .as_str()
            }
            _ => {
                return Err(self.report_error(&format!(
                    "Line[{}] Current token kind {} is not a identifier",
                    self.peek().unwrap().peek_line(),
                    self.peek_kind()
                )))
            }
        };

        Ok(String::from(ident))
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

            // sourceElement 只可能有两个 follow: { EOF, }(RightBracket) }
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
            TokenKind::KeyWord(KeyWordKind::Export) => {
                Ok(ASTNode::new(Stat::ExportStat(self.parse_export_stat()?)))
            }
            TokenKind::SemiColon => Ok(ASTNode::new(Stat::EmptyStat(self.parse_empty_stat()?))),

            // abstract class or abstract ?
            TokenKind::KeyWord(KeyWordKind::Abstract) => match self.look_ahead() {
                Some(TokenKind::KeyWord(KeyWordKind::Class)) => Ok(ASTNode::new(Stat::ClassDecl(
                    self.parse_class_declaration()?,
                ))),
                _ => Ok(ASTNode::new(Stat::AbsDecl(
                    self.parse_abstract_declaration()?,
                ))),
            },

            TokenKind::KeyWord(KeyWordKind::Class) => Ok(ASTNode::new(Stat::ClassDecl(
                self.parse_class_declaration()?,
            ))),

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
            // todo how to deal with exp declarations
            _ => {
                Err(self.report_error(&format!("Stat: Unexpected Token {}", self.peek().unwrap())))
            }
        }
    }

    fn parse_block(&mut self) -> Result<ASTNode<Block>, ParserError> {
        let mut block = Block::default();
        self.eat(TokenKind::LeftBracket)?;
        loop {
            match self.try_to(&Parser::parse_stat) {
                Some(stat) => block.push(stat),
                None => break,
            }
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
        if let Some(from_block) = self.try_to(&Parser::parse_from_block) {
            export_stat.set_from_block(from_block);
            return Ok(ASTNode::new(export_stat));
        }

        // 如果不是 from block, 那么说明一定是 stat
        // 之前的 match 保证进入这里面的 stat 一定不是 export 开头
        if let Some(from_block) = self.try_to(&Parser::parse_stat) {
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
    fn parse_class_declaration(&mut self) -> Result<ASTNode<ClassDecl>, ParserError> {
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

        class_decl.set_class_heritage(self.parse_class_heritage()?);
        class_decl.set_class_tail(self.parse_class_tail()?);
        Ok(ASTNode::new(class_decl))
    }

    fn parse_class_heritage(&mut self) -> Result<ASTNode<ClassHeritage>, ParserError> {
        let mut class_heritage = ClassHeritage::default();
        match self.peek_kind() {
            TokenKind::KeyWord(KeyWordKind::Extends)
            | TokenKind::KeyWord(KeyWordKind::Implements) => {
                if self.kind_is(TokenKind::KeyWord(KeyWordKind::Extends)) {
                    self.eat(TokenKind::KeyWord(KeyWordKind::Extends));
                    let extended_type = self.parse_type_ref()?;
                    class_heritage.set_extends(Extend::new(extended_type));
                }
                if self.kind_is(TokenKind::KeyWord(KeyWordKind::Implements)) {
                    let mut implemented = Implement::default();
                    self.eat(TokenKind::KeyWord(KeyWordKind::Implements));
                    loop {
                        let type_ref = self.parse_type_ref()?;
                        implemented.push_implemented(type_ref);
                        match self.peek_kind() {
                            TokenKind::Comma => {
                                self.eat(TokenKind::Comma);
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

    fn parse_type_generic(&mut self) -> Result<ASTNode<TypeGeneric>, ParserError> {
        Err(self.unsupported_error("type generic"))
    }

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

            // indexMemberDeclaration
            TokenKind::LeftBrace => Ok(ASTNode::new(ClassElement::IndexMemberDecl(
                self.parse_index_member_decl()?,
            ))),

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
                    self.parse_abstract_declaration()?,
                )));
            }

            _ => {
                if let Some(property_decl_exp) = self.try_to(&Parser::parse_property_decl_exp) {
                    return Ok(ASTNode::new(PropertyMemberDecl::PropertyDeclExp(
                        property_decl_exp,
                    )));
                }

                if let Some(method_declaration_exp) = self.try_to(&Parser::parse_method_decl_exp) {
                    return Ok(ASTNode::new(PropertyMemberDecl::MethodDeclExp(
                        method_declaration_exp,
                    )));
                }

                if let Some(gettersetter_decl_exp) =
                    self.try_to(&Parser::parse_gettersetter_decl_exp)
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

        if let Some(access_modifier) = self.try_to(&Parser::parse_access_modifier) {
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
            property_decl_exp.set_initializer(self.parse_single_exp()?);
        }

        self.eat(TokenKind::SemiColon)?;

        Ok(ASTNode::new(property_decl_exp))
    }

    /*
    accessibilityModifier? Static? Async? Identifier callSignature ( ('{' functionBody '}') | SemiColon )
        */
    fn parse_method_decl_exp(&mut self) -> Result<ASTNode<MethodDeclExp>, ParserError> {
        let mut method_decl_exp = MethodDeclExp::default();

        if let Some(access_modifier) = self.try_to(&Parser::parse_access_modifier) {
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
        method_decl_exp.set_call_sig(self.parse_call_signature()?);

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
        if let Some(access_modifier) = self.try_to(&Parser::parse_access_modifier) {
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

    fn parse_index_member_decl(&mut self) -> Result<ASTNode<IndexMemberDecl>, ParserError> {
        todo!()
    }

    fn parse_abstract_declaration(&mut self) -> Result<ASTNode<AbsDecl>, ParserError> {
        self.eat(TokenKind::KeyWord(KeyWordKind::Abstract))?;
        todo!()
    }

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
    fn parse_func_declaration(&mut self) -> Result<ASTNode<FuncDecl>, ParserError> {
        let mut func_decl = FuncDecl::default();

        // 函数声明
        self.eat(TokenKind::KeyWord(KeyWordKind::Function))?;
        func_decl.set_func_name(&self.extact_identifier()?);

        func_decl.set_call_sig(self.parse_call_signature()?);
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
    // Function_ '(' formalParameterList? ')' typeAnnotation? '{' functionBody '}';
    fn parse_func_exp_declaration(&mut self) -> Result<ASTNode<FuncExpDecl>, ParserError> {
        todo!()
    }

    /*
    callSignature:
        typeParameters? '(' parameterList? ')' typeAnnotation?;
    */
    fn parse_call_signature(&mut self) -> Result<ASTNode<CallSig>, ParserError> {
        let mut call_sig = CallSig::new();
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
            self.eat(TokenKind::Colon)?;
            call_sig.set_type_annotation(self.parse_type_annotation()?);
        }

        Ok(ASTNode::new(call_sig))
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
            self.eat(TokenKind::QuestionMark)?;
            formal_para.set_type(self.parse_type()?);
        }

        Ok(ASTNode::new(formal_para))
    }

    // singleExpression (',' singleExpression)*
    fn parse_exp_seq(&mut self) -> Result<ASTNode<ExpSeq>, ParserError> {
        let mut exp_seq = ExpSeq::default();
        loop {
            let single_exp = self.parse_single_exp()?;
            exp_seq.push_exp(single_exp);
            if !self.kind_is(TokenKind::Comma) {
                return Ok(ASTNode::new(exp_seq));
            }

            self.eat(TokenKind::Comma)?;
        }
    }

    fn parse_single_exp(&self) -> Result<ASTNode<Exp>, ParserError> {
        // TODO: this is first to deal with

        todo!()
    }

    fn parse_type_paras(&mut self) -> Result<ASTNode<TypeParas>, ParserError> {
        Err(self.unsupported_error("type paras"))
    }

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

    fn parse_type_annotation(&self) -> Result<ASTNode<TypeAnnotation>, ParserError> {
        todo!()
    }

    fn parse_rest_para(&self) -> Result<ASTNode<RestPara>, ParserError> {
        todo!()
    }

    fn parse_para(&mut self) -> Result<ASTNode<Para>, ParserError> {
        match self.peek_kind() {
            TokenKind::Identifier => {
                let mut para = Para::new();
                let para_name = self.peek().unwrap().peek_value();
                para.set_para_name(para_name);
                self.eat(TokenKind::Identifier)?;
                Ok(ASTNode::new(para))
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

    fn parse_type(&mut self) -> Result<ASTNode<Type>, ParserError> {
        todo!()
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

    // fn parse_eos(&mut self) -> Result<ASTNode<EOS>, ParserError> {
    //     if self.kind_is(TokenKind::SemiColon) {
    //         self.eat(TokenKind::SemiColon);
    //     } else if self.kind_is(TokenKind::EOF) {
    //         self.eat(TokenKind::EOF);
    //     }

    //     todo!()
    // }
}
