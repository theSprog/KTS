pub mod error;
mod exp_parser;
mod parser_util;

use crate::ast::ast_node::decorator::Decorators;
use crate::ast::ast_node::exp;
use crate::ast::ast_node::identifier;
use crate::ast::ast_node::identifier::Identifier;
use crate::ast::ast_node::literal::Literal;
use crate::ast::ast_node::program::Program;
use crate::ast::Span;

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

    // general error report
    fn report_error(&mut self, msg: &str) -> ParserError {
        let cur = self.peek().unwrap();
        if self.index < self.try_most_forward {
            // 拦截 msg, 换上更准确的错误。
            // 但是实际上，更准确的那个错误也是从 else 的那个分支传出来的
            return self.error_most_possible.clone().unwrap();
        }

        ParserError::new(format!(
            "{}: SyntaxError: near Line[{}]:\n{}",
            self.filename,
            cur.peek_line(),
            msg
        ))
    }

    fn mark_begin(&self) -> usize {
        self.peek().unwrap().peek_line()
    }

    fn mark_end(&self) -> usize {
        self.prepeek().peek_line()
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

    pub(crate) fn parse(&mut self) -> Result<AST, TSError> {
        Ok(AST::new(self.parse_program()?))
    }

    fn parse_program(&mut self) -> Result<ASTNode<Program>, ParserError> {
        let begin = self.mark_begin();
        let mut programe = Program::default();

        if self.kind_is(TokenKind::EOF) {
            return Ok(ASTNode::new(programe, Span::new(begin, begin)));
        }

        programe.set_source_elements(self.parse_source_elements()?);
        Ok(ASTNode::new(programe, Span::new(begin, self.mark_end())))
    }

    // sourceElements: sourceElement+;
    fn parse_source_elements(&mut self) -> Result<ASTNode<SourceElements>, ParserError> {
        let begin = self.mark_begin();
        let mut source_elements = SourceElements::default();

        loop {
            let source_element = self.parse_stat()?;
            source_elements.push_stat(source_element);

            // sourceElement 只可能有两个 follow: { EOF, "}" }
            if self.kind_is(TokenKind::EOF) || self.kind_is(TokenKind::RightBracket) {
                break;
            }
        }

        let end = self.mark_end();
        Ok(ASTNode::new(source_elements, Span::new(begin, end)))
    }

    fn parse_stat(&mut self) -> Result<ASTNode<Stat>, ParserError> {
        let begin = self.mark_begin();

        match self.peek_kind() {
            TokenKind::LeftBracket => {
                Ok(ASTNode::new(Stat::Block(self.parse_block()?), Span::new(begin, self.mark_end())))
            }
            
            TokenKind::KeyWord(KeyWordKind::Import) => {
                Ok(ASTNode::new(Stat::ImportStat(self.parse_import_stat()?), Span::new(begin, self.mark_end())))
            }

            TokenKind::KeyWord(KeyWordKind::Export) => match self.next_kind() {
                TokenKind::KeyWord(KeyWordKind::Declare)
                | TokenKind::KeyWord(KeyWordKind::Interface) => {
                    Ok(ASTNode::new(
                        Stat::InterfaceDecl(self.parse_interface_decl()?), Span::new(begin, self.mark_end())
                    ))
                }
                
                
                _ => {
                    Ok(ASTNode::new(Stat::ExportStat(self.parse_export_stat()?), Span::new(begin, self.mark_end())))
                }
                
                
            },

            TokenKind::SemiColon => Ok(ASTNode::new(Stat::EmptyStat(self.parse_empty_stat()?), Span::new(begin, self.mark_end()))),

            // abstract class or abstract ?
            TokenKind::KeyWord(KeyWordKind::Abstract) => match self.next_kind() {
                TokenKind::KeyWord(KeyWordKind::Class) => {
                    Ok(ASTNode::new(Stat::ClassDecl(self.parse_class_decl()?), Span::new(begin, self.mark_end())))
                }

                TokenKind::Identifier => {
                    Ok(ASTNode::new(Stat::AbsDecl(self.parse_abstract_decl()?), Span::new(begin, self.mark_end())))
                }

                _ => {
                    self.forward();
                    Err(self.expect_error("The abstract keyword can only modify class or identifier", "class or identifier"))
                }
            },

            TokenKind::KeyWord(KeyWordKind::Class) => {
                Ok(ASTNode::new(Stat::ClassDecl(self.parse_class_decl()?), Span::new(begin, self.mark_end())))
            }

            TokenKind::KeyWord(KeyWordKind::Interface) => Ok(ASTNode::new(Stat::InterfaceDecl(
                self.parse_interface_decl()?,
            ), Span::new(begin, self.mark_end()))),

            TokenKind::KeyWord(KeyWordKind::Namespace) => Ok(ASTNode::new(Stat::NamespaceDecl(
                self.parse_namespace_decl()?,
            ), Span::new(begin, self.mark_end()))),

            TokenKind::KeyWord(KeyWordKind::If) => {
                Ok(ASTNode::new(Stat::IfStat(self.parse_if_stat()?), Span::new(begin, self.mark_end())))
            }

            // do|while|for -> iteration stat
            TokenKind::KeyWord(KeyWordKind::Do)
            | TokenKind::KeyWord(KeyWordKind::While)
            | TokenKind::KeyWord(KeyWordKind::For) => {
                Ok(ASTNode::new(Stat::IterStat(self.parse_iter_stat()?), Span::new(begin, self.mark_end())))
            }

            TokenKind::KeyWord(KeyWordKind::Continue) => Ok(ASTNode::new(Stat::ContinueStat(self.parse_continue_stat()?), Span::new(begin, self.mark_end()))),
            TokenKind::KeyWord(KeyWordKind::Break) => Ok(ASTNode::new(Stat::BreakStat(self.parse_break_stat()?), Span::new(begin, self.mark_end()))),
            TokenKind::KeyWord(KeyWordKind::Return) => Ok(ASTNode::new(Stat::ReturnStat(self.parse_return_stat()?), Span::new(begin, self.mark_end()))),
            TokenKind::KeyWord(KeyWordKind::Yield) => Ok(ASTNode::new(Stat::YieldStat(self.parse_yield_stat()?), Span::new(begin, self.mark_end()))),
            TokenKind::KeyWord(KeyWordKind::With) => Ok(ASTNode::new(Stat::WithStat(self.parse_with_stat()?), Span::new(begin, self.mark_end()))),

            // TokenKind::Identifier => match self.lookAhead() {
            //     Some(TokenKind::Colon) => Ok(Some(self.parse_labelled_stat()?)),

            //     _ => Ok(Some(self.parse_abstract_declaration()?)),
            // },

            TokenKind::KeyWord(KeyWordKind::Switch) => Ok(ASTNode::new(Stat::SwitchStat(self.parse_switch_stat()?), Span::new(begin, self.mark_end()))),

            TokenKind::KeyWord(KeyWordKind::Throw) => Ok(ASTNode::new(Stat::ThrowStat(self.parse_throw_stat()?), Span::new(begin, self.mark_end()))),

            TokenKind::KeyWord(KeyWordKind::Try) => Ok(ASTNode::new(Stat::TryStat(self.parse_try_stat()?), Span::new(begin, self.mark_end()))),

            TokenKind::KeyWord(KeyWordKind::Debugger) => Ok(ASTNode::new(Stat::DebuggerStat(self.parse_debugger_stat()?), Span::new(begin, self.mark_end()))),

            // function 需要进一步往前探索
            TokenKind::KeyWord(KeyWordKind::Function) => match self.next_kind() {
                TokenKind::LeftParen => {
                    Ok(ASTNode::new(Stat::FuncExpDecl(self.parse_func_exp_decl()?), Span::new(begin, self.mark_end())))
                }

                TokenKind::Identifier => {
                    // func a<T>()
                    if self.lookahead(2) == TokenKind::LessThan {
                        Ok(ASTNode::new(Stat::FuncDecl(self.parse_func_decl()?), Span::new(begin, self.mark_end())))
                    }else {
                        // 至此，只有尝试了
                        if let Some(func_exp) = self.try_to(Parser::parse_func_exp_decl) {
                            Ok(ASTNode::new(Stat::FuncExpDecl(func_exp), Span::new(begin, self.mark_end())))
                        }else if let Some(func_decl) = self.try_to(Parser::parse_func_decl) {
                            Ok(ASTNode::new(Stat::FuncDecl(func_decl), Span::new(begin, self.mark_end())))
                        }else {
                            Err(self.expect_error("illegal function declartion", "function declaration or function expression"))
                        }
                    }
                }

                
                TokenKind::Multiply =>  Ok(ASTNode::new(Stat::GenFuncDecl(self.parse_generator_func_decl()?), Span::new(begin, self.mark_end()))),

                
                _ => {
                    self.forward();
                    Err(self.expect_error("illegal function declaretion", "Identifier, ( or *"))
                }
            },

            TokenKind::KeyWord(KeyWordKind::Enum) => {
                Ok(ASTNode::new(Stat::EnumStat(self.parse_enum_stat()?), Span::new(begin, self.mark_end())))
            }


            TokenKind:: KeyWord(KeyWordKind::Const) => {
                // const enum
                if self.nextkind_is(TokenKind::KeyWord(KeyWordKind::Enum)) {
                    Ok(ASTNode::new(Stat::EnumStat(self.parse_enum_stat()?), Span::new(begin, self.mark_end())))
                }
                // const var_stat
                else {
                    Ok(ASTNode::new(Stat::VarStat(self.parse_var_stat()?), Span::new(begin, self.mark_end())))
                }
            }

            TokenKind::KeyWord(KeyWordKind::Declare)
            | TokenKind:: KeyWord(KeyWordKind::Public)
            | TokenKind:: KeyWord(KeyWordKind::Protected)
            | TokenKind:: KeyWord(KeyWordKind::Private)
            | TokenKind:: KeyWord(KeyWordKind::Var)
            | TokenKind:: KeyWord(KeyWordKind::Let)
            | TokenKind:: KeyWord(KeyWordKind::ReadOnly) => {
                Ok(ASTNode::new(Stat::VarStat(self.parse_var_stat()?), Span::new(begin, self.mark_end())))
            }

            TokenKind::KeyWord(KeyWordKind::Type) => {
                Ok(ASTNode::new(Stat::TypeAliasStat(self.parse_typealias_stat()?), Span::new(begin, self.mark_end())))
            }

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
            | TokenKind::KeyWord(KeyWordKind::Null)
            | TokenKind::LeftBrace
            => {
                let exp_stat = self.parse_exp_seq()?;
                if self.kind_is(TokenKind::SemiColon) {
                    self.forward();
                }
                // ; 之后是 Identifier 并且是在同一行
                if ! self.prekind_is(TokenKind::SemiColon) && self.kind_is(TokenKind::Identifier) && ! self.is_new_line() {
                    return Err(self.report_error("maybe you forgot [,] to separate expression ?"))
                }
                Ok(ASTNode::new(Stat::ExpStat(exp_stat), Span::new(begin, self.mark_end())))
            }

            _ => Err(self.expect_error("Stat", "stat")),
        }
    }

    /*
    block: '{' statement* '}';
    */
    fn parse_block(&mut self) -> Result<ASTNode<Block>, ParserError> {
        let begin = self.mark_begin();

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
        Ok(ASTNode::new(block, Span::new(begin, self.mark_end())))
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
        let begin = self.mark_begin();

        self.eat(TokenKind::KeyWord(KeyWordKind::Import))?;

        if self.kind_is(TokenKind::Identifier) && self.nextkind_is(TokenKind::Assign) {
            let import_stat =
                ImportStat::new(ImportBlock::ImportAssign(self.set_import_alias_decl()?));
            Ok(ASTNode::new(import_stat, Span::new(begin, self.mark_end())))
        } else {
            let import_stat = ImportStat::new(ImportBlock::FromBlock(self.parse_from_block()?));
            Ok(ASTNode::new(import_stat, Span::new(begin, self.mark_end())))
        }
    }

    /*
    importAliasDeclaration
        : Identifier '=' namespaceName SemiColon
        ;
    */
    fn set_import_alias_decl(&mut self) -> Result<ASTNode<ImportAssign>, ParserError> {
        let begin = self.mark_begin();

        let identifier = self.parse_identifier()?;
        self.eat(TokenKind::Assign)?;
        let namespace_name = self.parse_namespace_name()?;
        let import_assign = ImportAssign::new(identifier, namespace_name);
        self.eat(TokenKind::SemiColon)?;
        Ok(ASTNode::new(
            import_assign,
            Span::new(begin, self.mark_end()),
        ))
    }

    /*
    namespaceDeclaration
        : Namespace namespaceName '{' statementList? '}'
        ;
    */
    fn parse_namespace_decl(&mut self) -> Result<ASTNode<NamespaceDecl>, ParserError> {
        let begin = self.mark_begin();

        let mut name_space_decl = NamespaceDecl::default();

        self.eat(TokenKind::KeyWord(KeyWordKind::Namespace))?;
        name_space_decl.set_name_space(self.parse_namespace_name()?);
        self.eat(TokenKind::LeftBracket)?;
        if !self.kind_is(TokenKind::RightBracket) {
            name_space_decl.set_source_elements(self.parse_source_elements()?);
        }
        self.eat(TokenKind::RightBracket)?;

        Ok(ASTNode::new(
            name_space_decl,
            Span::new(begin, self.mark_end()),
        ))
    }

    /*
    namespaceName
        : Identifier ('.'+ Identifier)*
        ;
    */
    fn parse_namespace_name(&mut self) -> Result<ASTNode<NamespaceName>, ParserError> {
        let begin = self.mark_begin();

        let mut name_space = NamespaceName::default();
        loop {
            let name = self.parse_identifier()?;
            name_space.push_name(name);
            if !self.kind_is(TokenKind::Dot) {
                break;
            }
            self.forward();
        }
        Ok(ASTNode::new(name_space, Span::new(begin, self.mark_end())))
    }

    fn parse_from_block(&mut self) -> Result<ASTNode<FromBlock>, ParserError> {
        let begin = self.mark_begin();

        let mut from_block = FromBlock::default();
        match self.peek_kind() {
            TokenKind::Multiply => {
                from_block.set_all();
                self.forward();
                if self.kind_is(TokenKind::KeyWord(KeyWordKind::As)) {
                    self.forward();
                    match self.peek_kind() {
                        TokenKind::Identifier => {
                            from_block.set_all_alias(self.parse_identifier()?);
                            if self.kind_is(TokenKind::Comma) {
                                self.forward();
                            }
                        }
                        _ => {
                            return Err(self.expect_error("illegal Import Statement", "Identifier"))
                        }
                    }
                }
            }
            TokenKind::Identifier | TokenKind::LeftBracket => {
                if self.kind_is(TokenKind::Identifier) {
                    from_block.set_imported(self.parse_identifier()?);

                    if self.kind_is(TokenKind::Comma) {
                        self.forward();
                    }
                }
                // if it be "{a, b as c, ...}"
                if self.kind_is(TokenKind::LeftBracket) {
                    self.eat(TokenKind::LeftBracket)?;

                    while self.kind_is(TokenKind::Identifier) {
                        let import_alias_begin = self.mark_begin();
                        let imported = self.parse_identifier()?;

                        if self.kind_is(TokenKind::KeyWord(KeyWordKind::As)) {
                            self.forward();
                            let alias = Some(self.parse_identifier()?);
                            let imported_alias = ASTNode::new(
                                PortedAlias::new(imported, alias),
                                Span::new(import_alias_begin, self.mark_end()),
                            );
                            from_block.push_imported_alias(imported_alias);
                        } else {
                            let imported_alias = ASTNode::new(
                                PortedAlias::new(imported, None),
                                Span::new(import_alias_begin, self.mark_end()),
                            );
                            from_block.push_imported_alias(imported_alias);
                        }

                        if self.kind_is(TokenKind::Comma) {
                            self.forward();
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
                let literal = self.extact_literal()?;
                from_block.set_from_value(ASTNode::new(literal, Span::new(begin, self.mark_end())));
            }
            _ => return Err(self.expect_error("Import Statement", "String Literal")),
        }

        self.eat_eos()?;
        Ok(ASTNode::new(from_block, Span::new(begin, self.mark_end())))
    }

    /*
        exportStatement: Export Default? (fromBlock | statement);
    */
    fn parse_export_stat(&mut self) -> Result<ASTNode<ExportStat>, ParserError> {
        let begin = self.mark_begin();

        let mut export_stat = ExportStat::default();

        self.eat(TokenKind::KeyWord(KeyWordKind::Export))?;
        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Default)) {
            export_stat.set_default();
            self.forward();
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
            return Ok(ASTNode::new(export_stat, Span::new(begin, self.mark_end())));
        }

        // 如果不是 from block, 那么说明一定是 stat
        // 之前的 match 保证进入这里面的 stat 一定不是 export 开头
        if let Some(stat) = self.try_to(Parser::parse_stat) {
            export_stat.set_stat(stat);
            self.eat_eos()?;
            return Ok(ASTNode::new(export_stat, Span::new(begin, self.mark_end())));
        }

        // 两个都不是, 出错
        Err(self.report_error("Expect [FromBlock] or [Statment] but there is no such match"))
    }

    fn parse_empty_stat(&mut self) -> Result<ASTNode<EmptyStat>, ParserError> {
        let begin = self.mark_begin();

        self.eat(TokenKind::SemiColon)?;
        Ok(ASTNode::new(
            EmptyStat::new(),
            Span::new(begin, self.mark_end()),
        ))
    }

    /*
    classDeclaration:
        Abstract? Class Identifier typeParameters? classHeritage classTail;

    for now typeParameters is not supported
    */
    fn parse_class_decl(&mut self) -> Result<ASTNode<ClassDecl>, ParserError> {
        let begin = self.mark_begin();

        let mut class_decl = ClassDecl::default();
        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Abstract)) {
            class_decl.set_abstract();
            self.eat(TokenKind::KeyWord(KeyWordKind::Abstract))?;
        }
        self.eat(TokenKind::KeyWord(KeyWordKind::Class))?;
        match self.peek_kind() {
            TokenKind::Identifier => {
                class_decl.set_class_name(self.parse_identifier()?);
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
        Ok(ASTNode::new(class_decl, Span::new(begin, self.mark_end())))
    }

    fn parse_class_heritage(&mut self) -> Result<ASTNode<ClassHeritage>, ParserError> {
        let begin = self.mark_begin();

        let mut class_heritage = ClassHeritage::default();
        match self.peek_kind() {
            TokenKind::KeyWord(KeyWordKind::Extends)
            | TokenKind::KeyWord(KeyWordKind::Implements) => {
                if self.kind_is(TokenKind::KeyWord(KeyWordKind::Extends)) {
                    self.forward();
                    let extended_type = self.parse_type_ref()?;
                    let extends = ASTNode::new(
                        Extends::new(extended_type),
                        Span::new(begin, self.mark_end()),
                    );
                    class_heritage.set_extends(extends);
                }
                if self.kind_is(TokenKind::KeyWord(KeyWordKind::Implements)) {
                    self.forward();
                    let mut implemented = Implement::default();
                    loop {
                        let type_ref = self.parse_type_ref()?;
                        implemented.push_implemented(type_ref);
                        if !self.kind_is(TokenKind::Comma) {
                            break;
                        }
                        self.forward();
                    }
                    let implemented = ASTNode::new(implemented, Span::new(begin, self.mark_end()));

                    class_heritage.set_implement(implemented);
                }
                Ok(ASTNode::new(
                    class_heritage,
                    Span::new(begin, self.mark_end()),
                ))
            }
            _ => return Err(self.expect_error("ClassHeritage Stat", "Extends or implements")),
        }
    }

    fn parse_type_ref(&mut self) -> Result<ASTNode<TypeRef>, ParserError> {
        let begin = self.mark_begin();

        let mut type_ref;

        match self.peek_kind() {
            TokenKind::Identifier => {
                if self.nextkind_is(TokenKind::Dot) {
                    type_ref = TypeRef::new_namespace(self.parse_namespace_name()?);
                } else {
                    type_ref = TypeRef::new_identifier(self.parse_identifier()?);
                }
                if self.kind_is(TokenKind::LessThan) {
                    type_ref.set_type_generic(self.parse_type_generic()?);
                }
                Ok(ASTNode::new(type_ref, Span::new(begin, self.mark_end())))
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
        let begin = self.mark_begin();

        let mut class_tail = ClassTail::default();
        self.eat(TokenKind::LeftBracket)?;

        loop {
            if self.kind_is(TokenKind::RightBracket) {
                break;
            }

            let class_element = self.parse_class_element()?;
            class_tail.push_class_element(class_element);
        }

        self.eat(TokenKind::RightBracket)?;
        return Ok(ASTNode::new(class_tail, Span::new(begin, self.mark_end())));
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
        let begin = self.mark_begin();

        match self.peek_kind() {
            // constructorDeclaration
            TokenKind::KeyWord(KeyWordKind::Constructor) => Ok(ASTNode::new(
                ClassElement::ConstructorDecl(self.parse_cons_decl()?),
                Span::new(begin, self.mark_end()),
            )),

            // propertyMemberDeclaration
            TokenKind::Identifier => Ok(ASTNode::new(
                ClassElement::PropertyMemberDecl(self.parse_property_member_decl()?),
                Span::new(begin, self.mark_end()),
            )),

            // propertyMemberDeclaration -> getter_setter_decl_exp
            TokenKind::KeyWord(KeyWordKind::Get) | TokenKind::KeyWord(KeyWordKind::Set) => {
                Ok(ASTNode::new(
                    ClassElement::PropertyMemberDecl(self.parse_property_member_decl()?),
                    Span::new(begin, self.mark_end()),
                ))
            }

            // indexMemberDeclaration
            TokenKind::LeftBrace => Ok(ASTNode::new(
                ClassElement::IndexMemberDecl(self.parse_index_member_decl()?),
                Span::new(begin, self.mark_end()),
            )),

            // propertyMemberDeclaration
            TokenKind::KeyWord(KeyWordKind::Async)
            | TokenKind::KeyWord(KeyWordKind::Static)
            | TokenKind::KeyWord(KeyWordKind::ReadOnly)
            | TokenKind::KeyWord(KeyWordKind::Abstract) => Ok(ASTNode::new(
                ClassElement::PropertyMemberDecl(self.parse_property_member_decl()?),
                Span::new(begin, self.mark_end()),
            )),

            TokenKind::KeyWord(KeyWordKind::Public)
            | TokenKind::KeyWord(KeyWordKind::Private)
            | TokenKind::KeyWord(KeyWordKind::Protected) => match self.next_kind() {
                // constructorDeclaration
                TokenKind::KeyWord(KeyWordKind::Constructor) => Ok(ASTNode::new(
                    ClassElement::ConstructorDecl(self.parse_cons_decl()?),
                    Span::new(begin, self.mark_end()),
                )),

                // propertyMemberDeclaration
                _ => Ok(ASTNode::new(
                    ClassElement::PropertyMemberDecl(self.parse_property_member_decl()?),
                    Span::new(begin, self.mark_end()),
                )),
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
        let begin = self.mark_begin();

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
        } else {
            let empty_formal_paras =
                ASTNode::new(FormalParas::default(), Span::new(begin, self.mark_end()));
            cons_decl.set_formal_paras(empty_formal_paras);
        }
        self.eat(TokenKind::RightParen)?;

        self.eat(TokenKind::LeftBracket)?;
        cons_decl.set_func_body(self.parse_func_body()?);
        self.eat(TokenKind::RightBracket)?;
        Ok(ASTNode::new(cons_decl, Span::new(begin, self.mark_end())))
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
        let begin = self.mark_begin();

        match self.peek_kind() {
            TokenKind::KeyWord(KeyWordKind::Abstract) => {
                // abstractDeclaration
                return Ok(ASTNode::new(
                    PropertyMemberDecl::AbsMemberDecl(self.parse_abstract_decl()?),
                    Span::new(begin, self.mark_end()),
                ));
            }

            _ => {
                if let Some(property_decl_exp) = self.try_to(Parser::parse_property_decl_exp) {
                    return Ok(ASTNode::new(
                        PropertyMemberDecl::PropertyDeclExp(property_decl_exp),
                        Span::new(begin, self.mark_end()),
                    ));
                }

                if let Some(method_declaration_exp) = self.try_to(Parser::parse_method_decl_exp) {
                    return Ok(ASTNode::new(
                        PropertyMemberDecl::MethodDeclExp(method_declaration_exp),
                        Span::new(begin, self.mark_end()),
                    ));
                }

                if let Some(gettersetter_decl_exp) =
                    self.try_to(Parser::parse_gettersetter_decl_exp)
                {
                    return Ok(ASTNode::new(
                        PropertyMemberDecl::GetterSetterDeclExp(gettersetter_decl_exp),
                        Span::new(begin, self.mark_end()),
                    ));
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
        let begin = self.mark_begin();

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

        property_decl_exp.set_identifier(self.parse_identifier()?);

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

        Ok(ASTNode::new(
            property_decl_exp,
            Span::new(begin, self.mark_end()),
        ))
    }

    /*
    accessibilityModifier? Static? Async? Identifier callSignature ( ('{' functionBody '}') | SemiColon )
        */
    fn parse_method_decl_exp(&mut self) -> Result<ASTNode<MethodDeclExp>, ParserError> {
        let begin = self.mark_begin();

        let mut method_decl_exp = MethodDeclExp::default();

        if let Some(access_modifier) = self.try_to(Parser::parse_access_modifier) {
            method_decl_exp.set_access_modifier(access_modifier);
        }

        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Static)) {
            self.forward();
            method_decl_exp.set_static();
        }
        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Async)) {
            self.forward();
            method_decl_exp.set_async();
        }

        method_decl_exp.set_identifier(self.parse_identifier()?);
        method_decl_exp.set_call_sig(self.parse_call_sig()?);

        match self.peek_kind() {
            TokenKind::LeftBracket => {
                self.eat(TokenKind::LeftBracket)?;
                method_decl_exp.set_func_body(self.parse_func_body()?);
                self.eat(TokenKind::RightBracket)?;
            }
            TokenKind::SemiColon => {
                self.forward();
                return Ok(ASTNode::new(
                    method_decl_exp,
                    Span::new(begin, self.mark_end()),
                ));
            }

            _ => {
                return Err(self.expect_error("Method Declaration Expression", "{ funcbody } or ;"));
            }
        }

        Ok(ASTNode::new(
            method_decl_exp,
            Span::new(begin, self.mark_end()),
        ))
    }

    /*
    accessibilityModifier? Static? (getAccessor | setAccessor)
    */
    fn parse_gettersetter_decl_exp(&mut self) -> Result<ASTNode<GetterSetterDeclExp>, ParserError> {
        let begin = self.mark_begin();
        // it is ugly, but it is necessary
        let mut access_modifier_ = None;
        let mut static_ = false;
        if let Some(access_modifier) = self.try_to(Parser::parse_access_modifier) {
            access_modifier_ = Some(access_modifier);
        }
        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Static)) {
            static_ = true;
            self.forward();
        }

        let getter_setter_decl_exp =
            GetterSetterDeclExp::new(access_modifier_, static_, self.parse_accesser()?);
        Ok(ASTNode::new(
            getter_setter_decl_exp,
            Span::new(begin, self.mark_end()),
        ))
    }

    fn parse_accesser(&mut self) -> Result<ASTNode<Accesser>, ParserError> {
        let begin = self.mark_begin();

        match self.peek_kind() {
            TokenKind::KeyWord(KeyWordKind::Get) => {
                let mut accesser = GetAccesser::default();
                self.forward();
                accesser.set_identifier(self.parse_identifier()?);
                self.eat(TokenKind::LeftParen)?;
                self.eat(TokenKind::RightParen)?;
                if self.kind_is(TokenKind::Colon) {
                    accesser.set_type_annotation(self.parse_type_annotation()?);
                }
                if self.kind_is(TokenKind::SemiColon) {
                    self.forward();
                    Ok(ASTNode::new(
                        Accesser::GetAccessor(accesser),
                        Span::new(begin, self.mark_end()),
                    ))
                } else {
                    self.eat(TokenKind::LeftBracket)?;
                    accesser.set_func_body(self.parse_func_body()?);
                    self.eat(TokenKind::RightBracket)?;
                    self.eat_eos()?;
                    Ok(ASTNode::new(
                        Accesser::GetAccessor(accesser),
                        Span::new(begin, self.mark_end()),
                    ))
                }
            }
            TokenKind::KeyWord(KeyWordKind::Set) => {
                let mut accesser = SetAccesser::default();
                self.forward();
                accesser.set_identifier(self.parse_identifier()?);

                self.eat(TokenKind::LeftParen)?;
                accesser.set_parameter(self.parse_identifier()?);
                if self.kind_is(TokenKind::Colon) {
                    accesser.set_type_annotation(self.parse_type_annotation()?);
                } else if self.kind_is(TokenKind::Assign) {
                    return Err(self.unsupported_error("Set Accesser default parameter"));
                }
                self.eat(TokenKind::RightParen)?;

                if self.kind_is(TokenKind::SemiColon) {
                    self.forward();
                    Ok(ASTNode::new(
                        Accesser::SetAccessor(accesser),
                        Span::new(begin, self.mark_end()),
                    ))
                } else {
                    self.eat(TokenKind::LeftBracket)?;
                    accesser.set_func_body(self.parse_func_body()?);
                    self.eat(TokenKind::RightBracket)?;
                    self.eat_eos()?;
                    Ok(ASTNode::new(
                        Accesser::SetAccessor(accesser),
                        Span::new(begin, self.mark_end()),
                    ))
                }
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
        let begin = self.mark_begin();

        let index_sig = self.parse_index_sig()?;
        self.eat(TokenKind::SemiColon)?;
        Ok(ASTNode::new(
            IndexMemberDecl::new(index_sig),
            Span::new(begin, self.mark_end()),
        ))
    }

    /*
    indexSignature:
        '[' Identifier ':' (Number | String) ']' typeAnnotation;
    */
    fn parse_index_sig(&mut self) -> Result<ASTNode<IndexSig>, ParserError> {
        let begin = self.mark_begin();

        let type_;
        self.eat(TokenKind::LeftBrace)?;
        let index_name = self.parse_identifier()?;
        self.eat(TokenKind::Colon)?;

        let begin2 = self.mark_begin();
        match self.peek_kind() {
            TokenKind::KeyWord(KeyWordKind::Number) => {
                type_ = Some(ASTNode::new(
                    PredefinedType::Number,
                    Span::new(begin2, self.mark_end()),
                ));
                self.forward();
            }
            TokenKind::KeyWord(KeyWordKind::String) => {
                type_ = Some(ASTNode::new(
                    PredefinedType::String,
                    Span::new(begin2, self.mark_end()),
                ));
                self.forward();
            }
            _ => return Err(self.expect_error("Index Signature", "Number or String")),
        };
        self.eat(TokenKind::RightBrace)?;
        let type_annotation = self.parse_type_annotation()?;

        Ok(ASTNode::new(
            IndexSig::new(index_name, type_, type_annotation),
            Span::new(begin, self.mark_end()),
        ))
    }

    /*
        abstractDeclaration: Abstract (AbstratcMethod | AbstratcVar | (getAccessor | setAccessor)) eos;

        AbstratcMethod:
            Identifier callSignature
        AbstratcVar:
            variableStatement
    */
    fn parse_abstract_decl(&mut self) -> Result<ASTNode<AbsDecl>, ParserError> {
        let begin = self.mark_begin();

        let abs_method;
        self.eat(TokenKind::KeyWord(KeyWordKind::Abstract))?;

        let begin1 = self.mark_begin();
        if self.kind_is(TokenKind::Identifier) && self.nextkind_is(TokenKind::LeftParen) {
            let identifier = self.parse_identifier()?;
            let call_sig = self.parse_call_sig()?;
            abs_method = AbsMember::AbsMethod(ASTNode::new(
                AbsMethod::new(identifier, call_sig),
                Span::new(begin1, self.mark_end()),
            ));
        } else if self.kind_is(TokenKind::KeyWord(KeyWordKind::Get))
            || self.kind_is(TokenKind::KeyWord(KeyWordKind::Set))
        {
            abs_method = AbsMember::AbsAccesser(self.parse_accesser()?);
        } else {
            abs_method = AbsMember::AbsVar(ASTNode::new(
                AbsVar::new(self.parse_var_stat()?),
                Span::new(begin1, self.mark_end()),
            ));
        }

        self.eat_eos()?;
        let abs_method = ASTNode::new(abs_method, Span::new(begin, self.mark_end()));
        Ok(ASTNode::new(
            AbsDecl::new(abs_method),
            Span::new(begin, self.mark_end()),
        ))
    }

    /*
    ifStatement:
        If '(' expressionSequence ')' statement (Else statement)?;
    */
    fn parse_if_stat(&mut self) -> Result<ASTNode<IfStat>, ParserError> {
        let begin = self.mark_begin();

        let mut if_stat = IfStat::default();
        self.eat(TokenKind::KeyWord(KeyWordKind::If))?;

        self.eat(TokenKind::LeftParen)?;
        if_stat.set_exp_seq(self.parse_exp_seq()?);
        self.eat(TokenKind::RightParen)?;

        if_stat.set_stat(self.parse_stat()?);

        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Else)) {
            self.forward();
            if_stat.set_else_stat(self.parse_stat()?);
        }
        Ok(ASTNode::new(if_stat, Span::new(begin, self.mark_end())))
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
        let begin = self.mark_begin();

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
                let do_stat =
                    ASTNode::new(DoStat::new(stat, exp), Span::new(begin, self.mark_end()));
                Ok(ASTNode::new(
                    IterStat::DoStat(do_stat),
                    Span::new(begin, self.mark_end()),
                ))
            }

            // While '(' singleExpression ')' statement		# WhileStatement
            TokenKind::KeyWord(KeyWordKind::While) => {
                self.eat(TokenKind::KeyWord(KeyWordKind::While))?;

                self.eat(TokenKind::LeftParen)?;
                let exp = self.parse_exp()?;
                self.eat(TokenKind::RightParen)?;
                let stat = self.parse_stat()?;
                let while_stat =
                    ASTNode::new(WhileStat::new(exp, stat), Span::new(begin, self.mark_end()));

                Ok(ASTNode::new(
                    IterStat::WhileStat(while_stat),
                    Span::new(begin, self.mark_end()),
                ))
            }

            TokenKind::KeyWord(KeyWordKind::For) => {
                if let Some(for_stat) = self.try_to(Parser::parse_for_stat) {
                    return Ok(ASTNode::new(
                        IterStat::ForStat(for_stat),
                        Span::new(begin, self.mark_end()),
                    ));
                }

                if let Some(forin_stat) = self.try_to(Parser::parse_forin_stat) {
                    return Ok(ASTNode::new(
                        IterStat::ForInStat(forin_stat),
                        Span::new(begin, self.mark_end()),
                    ));
                }

                if let Some(forvar_stat) = self.try_to(Parser::parse_forvar_stat) {
                    return Ok(ASTNode::new(
                        IterStat::ForVarStat(forvar_stat),
                        Span::new(begin, self.mark_end()),
                    ));
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
        let begin = self.mark_begin();

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
        Ok(ASTNode::new(for_stat, Span::new(begin, self.mark_end())))
    }

    /*
    For '(' identifier In expression ')' statement	# ForInStatement;
    */
    fn parse_forin_stat(&mut self) -> Result<ASTNode<ForInStat>, ParserError> {
        let begin = self.mark_begin();
        let var;
        let exp;
        let stat;
        self.eat(TokenKind::KeyWord(KeyWordKind::For))?;
        self.eat(TokenKind::LeftParen)?;

        let ident_begin = self.mark_begin();
        let identifier = Exp::Identifier(ASTNode::new(
            Identifier::new(&self.extact_identifier()?),
            Span::new(ident_begin, self.mark_end()),
        ));
        var = ASTNode::new(identifier, Span::new(ident_begin, self.mark_end()));

        self.eat(TokenKind::KeyWord(KeyWordKind::In))?;
        exp = self.parse_exp()?;
        self.eat(TokenKind::RightParen)?;
        stat = self.parse_stat()?;
        Ok(ASTNode::new(
            ForInStat::new(var, exp, stat),
            Span::new(begin, self.mark_end()),
        ))
    }

    /*
    For '(' varModifier variableDeclarationList SemiColon expression? SemiColon expressionSequence? ')' statement							# ForVarStatement
    )?;
    */
    fn parse_forvar_stat(&mut self) -> Result<ASTNode<ForVarStat>, ParserError> {
        let begin = self.mark_begin();

        let var_modifier;
        let var_decl_list;
        let mut cond = None;
        let mut action = None;
        let stat;

        self.eat(TokenKind::KeyWord(KeyWordKind::For))?;
        self.eat(TokenKind::LeftParen)?;

        var_modifier = self.parse_var_modifier()?;
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

        Ok(ASTNode::new(
            ForVarStat::new(var_modifier, var_decl_list, cond, action, stat),
            Span::new(begin, self.mark_end()),
        ))
    }

    /*
    variableDeclarationList: variableDeclaration (',' variableDeclaration)*;
    */
    fn parse_var_decl_list(&mut self) -> Result<ASTNode<VarDeclList>, ParserError> {
        let begin = self.mark_begin();

        let mut var_decl_list = VarDeclList::default();
        loop {
            var_decl_list.push_var_decl(self.parse_var_decl()?);
            if !self.kind_is(TokenKind::Comma) {
                break;
            }
            self.eat(TokenKind::Comma)?;
        }
        Ok(ASTNode::new(
            var_decl_list,
            Span::new(begin, self.mark_end()),
        ))
    }

    /*
    variableDeclaration: (Identifier | arrayLiteral | objectLiteral) typeAnnotation? ('=' expression)?;
    */
    fn parse_var_decl(&mut self) -> Result<ASTNode<VarDecl>, ParserError> {
        let begin = self.mark_begin();

        match self.peek_kind() {
            TokenKind::Identifier => {
                let mut var_decl = VarDecl::new(self.parse_identifier()?);
                if self.kind_is(TokenKind::Colon) {
                    var_decl.set_type_annotation(self.parse_type_annotation()?);
                }
                if self.kind_is(TokenKind::Assign) {
                    self.forward();
                    var_decl.set_initializer(self.parse_exp()?);
                }
                Ok(ASTNode::new(var_decl, Span::new(begin, self.mark_end())))
            }
            TokenKind::LeftBrace => Err(self.unsupported_error("Array Matching")),
            TokenKind::LeftBracket => Err(self.unsupported_error("Object Matching")),
            _ => Err(self.expect_error("Var Decl", "Identifier or arrayLiteral or objectLiteral")),
        }
    }

    /*
    continueStatement:
        Continue (Identifier)? eos;
    */
    fn parse_continue_stat(&mut self) -> Result<ASTNode<ContinueStat>, ParserError> {
        let begin = self.mark_begin();

        let mut continue_stat = ContinueStat::default();
        self.eat(TokenKind::KeyWord(KeyWordKind::Continue))?;
        if self.kind_is(TokenKind::Identifier) {
            continue_stat.set_identifier(self.parse_identifier()?);
        }
        let eos = self.eat_eos()?;
        return Ok(ASTNode::new(
            continue_stat,
            Span::new(begin, self.mark_end()),
        ));
    }

    /*
    breakStatement:
        Break (Identifier)? eos;
    */
    fn parse_break_stat(&mut self) -> Result<ASTNode<BreakStat>, ParserError> {
        let begin = self.mark_begin();

        let mut break_stat = BreakStat::default();
        self.eat(TokenKind::KeyWord(KeyWordKind::Break))?;

        if self.kind_is(TokenKind::Identifier) {
            break_stat.set_identifier(self.parse_identifier()?);
        }
        let eos = self.eat_eos()?;
        return Ok(ASTNode::new(break_stat, Span::new(begin, self.mark_end())));
    }

    /*
    Return (expressionSequence)? eos;
    */
    fn parse_return_stat(&mut self) -> Result<ASTNode<ReturnStat>, ParserError> {
        let begin = self.mark_begin();

        let mut return_stat = ReturnStat::default();
        self.eat(TokenKind::KeyWord(KeyWordKind::Return))?;
        if !self.is_eos() {
            return_stat.set_exp_seq(self.parse_exp_seq()?);
        }
        self.eat_eos()?;
        Ok(ASTNode::new(return_stat, Span::new(begin, self.mark_end())))
    }

    /*
    yieldStatement: Yield (expressionSequence)? eos;
    */
    fn parse_yield_stat(&mut self) -> Result<ASTNode<YieldStat>, ParserError> {
        let begin = self.mark_begin();

        let mut yield_stat = YieldStat::default();
        self.eat(TokenKind::KeyWord(KeyWordKind::Yield))?;
        yield_stat.set_exp_seq(self.parse_exp_seq()?);
        let eos = self.eat_eos()?;
        Ok(ASTNode::new(yield_stat, Span::new(begin, self.mark_end())))
    }

    //    : With '(' expressionSequence ')' statement
    fn parse_with_stat(&mut self) -> Result<ASTNode<WithStat>, ParserError> {
        let begin = self.mark_begin();

        self.eat(TokenKind::KeyWord(KeyWordKind::With))?;
        self.eat(TokenKind::LeftParen)?;
        let exp_seq = self.parse_exp_seq()?;
        self.eat(TokenKind::RightParen)?;
        let stat = self.parse_stat()?;
        Ok(ASTNode::new(
            WithStat::new(exp_seq, stat),
            Span::new(begin, self.mark_end()),
        ))
    }

    // Identifier ':' statement
    fn parse_labelled_stat(&mut self) -> Result<ASTNode<LabelledStat>, ParserError> {
        let begin = self.mark_begin();

        let identifier = &self.extact_identifier()?;
        self.eat(TokenKind::Colon)?;
        let stat = self.parse_stat()?;
        let identifier = self.parse_identifier()?;
        Ok(ASTNode::new(
            LabelledStat::new(identifier, stat),
            Span::new(begin, self.mark_end()),
        ))
    }

    // Switch '(' expression ')' caseBlock
    fn parse_switch_stat(&mut self) -> Result<ASTNode<SwitchStat>, ParserError> {
        let begin = self.mark_begin();

        self.eat(TokenKind::KeyWord(KeyWordKind::Switch))?;
        self.eat(TokenKind::LeftParen)?;
        let exp = self.parse_exp()?;
        self.eat(TokenKind::RightParen)?;
        let cases_block = self.parse_case_block()?;
        Ok(ASTNode::new(
            SwitchStat::new(exp, cases_block),
            Span::new(begin, self.mark_end()),
        ))
    }

    /*
    caseBlock: '{' caseClauses? (defaultClause)? '}';
    */
    fn parse_case_block(&mut self) -> Result<ASTNode<CaseBlock>, ParserError> {
        let begin = self.mark_begin();

        let mut case_block = CaseBlock::default();
        self.eat(TokenKind::LeftBracket)?;
        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Case)) {
            case_block.set_case_clauses(self.parse_case_clauses()?);
        }

        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Default)) {
            case_block.set_default_clause(self.parse_default_clause()?);
        }
        self.eat(TokenKind::RightBracket)?;
        Ok(ASTNode::new(case_block, Span::new(begin, self.mark_end())))
    }

    // caseClause+
    fn parse_case_clauses(&mut self) -> Result<ASTNode<CaseClauses>, ParserError> {
        let begin = self.mark_begin();

        let mut case_clauses = CaseClauses::default();
        loop {
            match self.peek_kind() {
                TokenKind::KeyWord(KeyWordKind::Case) => {
                    case_clauses.push_case_clause(self.parse_case_clause()?)
                }
                _ => {
                    return Ok(ASTNode::new(
                        case_clauses,
                        Span::new(begin, self.mark_end()),
                    ))
                }
            }
        }
    }

    /*
     * Case expression ':' sourceElements?
     */
    fn parse_case_clause(&mut self) -> Result<ASTNode<CaseClause>, ParserError> {
        let begin = self.mark_begin();

        let mut stats = None;
        self.eat(TokenKind::KeyWord(KeyWordKind::Case))?;
        let exp = self.parse_exp()?;
        self.eat(TokenKind::Colon)?;
        if !self.kind_is(TokenKind::KeyWord(KeyWordKind::Case)) {
            stats = Some(self.parse_source_elements()?);
        }
        Ok(ASTNode::new(
            CaseClause::new(exp, stats),
            Span::new(begin, self.mark_end()),
        ))
    }

    /*
    defaultClause: Default ':' sourceElements?;
    */
    fn parse_default_clause(&mut self) -> Result<ASTNode<DefaultClause>, ParserError> {
        let begin = self.mark_begin();

        let mut stats = None;
        self.eat(TokenKind::KeyWord(KeyWordKind::Default))?;
        self.eat(TokenKind::Colon)?;
        if !self.kind_is(TokenKind::RightBracket) {
            stats = Some(self.parse_source_elements()?);
        }
        Ok(ASTNode::new(
            DefaultClause::new(stats),
            Span::new(begin, self.mark_end()),
        ))
    }

    // Throw {this.notLineTerminator()}? expressionSequence eos
    fn parse_throw_stat(&mut self) -> Result<ASTNode<ThrowStat>, ParserError> {
        let begin = self.mark_begin();
        self.eat(TokenKind::KeyWord(KeyWordKind::Throw))?;
        let exp_seq = self.parse_exp_seq()?;
        let eos = self.eat_eos()?;
        todo!()
    }

    // Try block (catchProduction finallyProduction? | finallyProduction)
    fn parse_try_stat(&mut self) -> Result<ASTNode<TryStat>, ParserError> {
        let mut try_stat = TryStat::default();
        self.eat(TokenKind::KeyWord(KeyWordKind::Try))?;
        try_stat.set_block(self.parse_block()?);
        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Catch)) {
            todo!()
        }
        todo!()
    }

    fn parse_debugger_stat(&mut self) -> Result<ASTNode<DebuggerStat>, ParserError> {
        Err(self.unsupported_error("debugger"))
    }

    /*
    functionDeclaration
        : Function Identifier callSignature ( ('{' functionBody '}') | SemiColon);
    */
    fn parse_func_decl(&mut self) -> Result<ASTNode<FuncDecl>, ParserError> {
        let begin = self.mark_begin();

        let mut func_body = None;
        self.eat(TokenKind::KeyWord(KeyWordKind::Function))?;
        let func_name = self.parse_identifier()?;
        let call_sig = self.parse_call_sig()?;
        if self.kind_is(TokenKind::LeftBracket) {
            self.eat(TokenKind::LeftBracket)?;
            func_body = Some(self.parse_func_body()?);
            self.eat(TokenKind::RightBracket)?;
        } else {
            self.eat(TokenKind::SemiColon)?;
        }

        let func_decl = FuncDecl::new(func_name, call_sig, func_body);
        Ok(ASTNode::new(func_decl, Span::new(begin, self.mark_end())))
    }

    // functionExpressionDeclaration:
    // Function_ Identifier? '(' formalParameterList? ')' typeAnnotation? '{' functionBody '}';
    fn parse_func_exp_decl(&mut self) -> Result<ASTNode<FuncExpDecl>, ParserError> {
        let begin = self.mark_begin();

        let mut func_exp_decl = FuncExpDecl::default();
        self.eat(TokenKind::KeyWord(KeyWordKind::Function))?;
        if self.kind_is(TokenKind::Identifier) {
            func_exp_decl.set_func_name(self.parse_identifier()?);
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

        Ok(ASTNode::new(
            func_exp_decl,
            Span::new(begin, self.mark_end()),
        ))
    }

    /*
    arrowFunctionDeclaration:
        Async? '(' formalParameterList? ')' typeAnnotation? '=>' arrowFunctionBody;
    */
    fn parse_arrow_func(&mut self) -> Result<ASTNode<ArrowFuncExpDecl>, ParserError> {
        let begin = self.mark_begin();

        let mut arrow_func = ArrowFuncExpDecl::default();
        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Async)) {
            self.forward();
            arrow_func.set_async();
        }

        let para_begin = self.mark_begin();
        match self.peek_kind() {
            TokenKind::LeftParen => {
                self.eat(TokenKind::LeftParen)?;
                if !self.kind_is(TokenKind::RightParen) {
                    arrow_func.set_formal_paras(self.parse_formal_parameters()?);
                } else {
                    arrow_func.set_formal_paras(ASTNode::new(
                        FormalParas::default(),
                        Span::new(para_begin, self.mark_end()),
                    ));
                }
                self.eat(TokenKind::RightParen)?;
            }
            TokenKind::Identifier => {
                let mut formal_para = FormalPara::default();
                formal_para.set_identifier(self.parse_identifier()?);

                let mut formal_paras = FormalParas::default();
                formal_paras
                    .push_formal_para(ASTNode::new(formal_para, Span::new(begin, self.mark_end())));

                arrow_func.set_formal_paras(ASTNode::new(
                    formal_paras,
                    Span::new(begin, self.mark_end()),
                ));
            }

            _ => return Err(self.expect_error("arrow function", "identifier or (")),
        }

        if self.kind_is(TokenKind::Colon) {
            arrow_func.set_type_annotation(self.parse_type_annotation()?);
        }
        self.eat(TokenKind::ARROW)?;
        if self.kind_is(TokenKind::LeftBracket) {
            self.eat(TokenKind::LeftBracket)?;

            let func_body = ASTNode::new(
                ArrowFuncBody::FuncBody(self.parse_func_body()?),
                Span::new(begin, self.mark_end()),
            );
            arrow_func.set_func_body(func_body);

            self.eat(TokenKind::RightBracket)?;
        } else {
            let exp_body = ASTNode::new(
                ArrowFuncBody::ExpBody(self.parse_exp()?),
                Span::new(begin, self.mark_end()),
            );
            arrow_func.set_func_body(exp_body);
        }

        Ok(ASTNode::new(arrow_func, Span::new(begin, self.mark_end())))
    }

    /*
    callSignature:
        typeParameters? '(' parameterList? ')' typeAnnotation?;
    */
    fn parse_call_sig(&mut self) -> Result<ASTNode<CallSig>, ParserError> {
        let begin = self.mark_begin();

        let mut call_sig = CallSig::default();
        if self.kind_is(TokenKind::LessThan) {
            call_sig.set_type_paras(self.parse_type_paras()?);
        }
        self.eat(TokenKind::LeftParen)?;

        let paras_begin = self.mark_begin();
        match self.peek_kind() {
            TokenKind::Ellipsis
            | TokenKind::Identifier
            | TokenKind::LeftBracket
            | TokenKind::LeftBrace => call_sig.set_para_list(self.parse_para_list()?),

            // 即使 () 内什么也没有，也要去申请结点。
            // 因为内容和结点是分离的
            _ => call_sig.set_para_list(ASTNode::new(
                ParaList::default(),
                Span::new(paras_begin, self.mark_end()),
            )),
        }
        self.eat(TokenKind::RightParen)?;
        if self.kind_is(TokenKind::Colon) {
            call_sig.set_type_annotation(self.parse_type_annotation()?);
        }

        Ok(ASTNode::new(call_sig, Span::new(begin, self.mark_end())))
    }

    /*
    constructSignature:
        'new' typeParameters? '(' parameterList? ')' typeAnnotation?;
    */
    fn parse_construct_sig(&mut self) -> Result<ASTNode<ConstructSig>, ParserError> {
        let begin = self.mark_begin();

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

        Ok(ASTNode::new(
            construct_sig,
            Span::new(begin, self.mark_end()),
        ))
    }

    /*
    propertySignature:
        ReadOnly? Identifier '?'? typeAnnotation?;
    */
    fn parse_property_sig(&mut self) -> Result<ASTNode<PropertySig>, ParserError> {
        let begin = self.mark_begin();

        let mut property_sig = PropertySig::default();
        if self.kind_is(TokenKind::KeyWord(KeyWordKind::ReadOnly)) {
            property_sig.set_readonly();
            self.forward();
        }

        property_sig.set_property_name(self.parse_identifier()?);

        if self.kind_is(TokenKind::QuestionMark) {
            property_sig.set_question_mark();
            self.forward();
        }

        if self.kind_is(TokenKind::Colon) {
            property_sig.set_type_annotation(self.parse_type_annotation()?);
        }

        Ok(ASTNode::new(
            property_sig,
            Span::new(begin, self.mark_end()),
        ))
    }

    /*
    methodSignature: Identifier '?'? callSignature;
    */
    fn parse_method_sig(&mut self) -> Result<ASTNode<MethodSig>, ParserError> {
        let begin = self.mark_begin();

        let mut method_sig = MethodSig::default();
        method_sig.set_method_name(self.parse_identifier()?);
        if self.kind_is(TokenKind::QuestionMark) {
            method_sig.set_question_mark();
            self.forward();
        }

        method_sig.set_call_sig(self.parse_call_sig()?);

        Ok(ASTNode::new(method_sig, Span::new(begin, self.mark_end())))
    }

    /*
    functionBody
        : sourceElements?
        ;
    functionBody 左右必是被 { } 包围
    */
    fn parse_func_body(&mut self) -> Result<ASTNode<FuncBody>, ParserError> {
        // prekind is {
        let begin = self.prepeek().peek_line();

        let mut func_body = FuncBody::default();

        if self.kind_is(TokenKind::RightBracket) {
            let end = self.peek().unwrap().peek_line();
            return Ok(ASTNode::new(func_body, Span::new(begin, end)));
        }
        func_body.set_func_body(self.parse_source_elements()?);

        // current kind is }
        let end = self.peek().unwrap().peek_line();
        Ok(ASTNode::new(func_body, Span::new(begin, end)))
    }

    /*
    生成器函数声明
    Function_ '*' Identifier? '(' formalParameterList? ')' '{' functionBody '}'
     */
    fn parse_generator_func_decl(&mut self) -> Result<ASTNode<GenFuncDecl>, ParserError> {
        let begin = self.mark_begin();

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
        let begin = self.mark_begin();

        let mut formal_paras = FormalParas::default();

        if self.kind_is(TokenKind::Ellipsis) {
            self.forward();
            formal_paras.set_last_para_arg(self.parse_identifier()?);
        } else {
            loop {
                let formal_parameter_arg = self.parse_formal_parameter_arg()?;
                formal_paras.push_formal_para(formal_parameter_arg);
                match self.peek_kind() {
                    TokenKind::Comma => {
                        self.eat(TokenKind::Comma)?;
                        if self.kind_is(TokenKind::Ellipsis) {
                            self.forward();
                            formal_paras.set_last_para_arg(self.parse_identifier()?);
                            break;
                        }
                    }
                    _ => break,
                }
            }
        }

        Ok(ASTNode::new(
            formal_paras,
            Span::new(begin, self.mark_end()),
        ))
    }

    /*
    formalParameterArg:
        decorator? accessibilityModifier? Identifier '?'? typeAnnotation?;
    */
    fn parse_formal_parameter_arg(&mut self) -> Result<ASTNode<FormalPara>, ParserError> {
        let begin = self.mark_begin();

        let mut formal_para = FormalPara::default();
        if self.kind_is(TokenKind::At) {
            formal_para.set_decorator();
            self.forward();
        }

        if let Some(access_modifier) = self.try_to(Parser::parse_access_modifier) {
            formal_para.set_access_modifier(self.parse_access_modifier()?);
        }

        formal_para.set_identifier(self.parse_identifier()?);

        if self.kind_is(TokenKind::QuestionMark) {
            formal_para.set_question_mark();
            self.forward();
        }

        if self.kind_is(TokenKind::Colon) {
            formal_para.set_type_annotation(self.parse_type_annotation()?);
        }

        Ok(ASTNode::new(formal_para, Span::new(begin, self.mark_end())))
    }

    // expression (',' expression)*
    fn parse_exp_seq(&mut self) -> Result<ASTNode<ExpSeq>, ParserError> {
        let begin = self.mark_begin();
        let mut exp_seq = ExpSeq::default();
        loop {
            let exp = self.parse_exp()?;
            exp_seq.push_exp(exp);
            if !self.kind_is(TokenKind::Comma) {
                return Ok(ASTNode::new(exp_seq, Span::new(begin, self.mark_end())));
            }
            self.forward();
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
        let begin = self.mark_begin();

        let mut para_list = ParaList::default();

        match self.peek_kind() {
            TokenKind::Ellipsis => {
                let rest_para = self.parse_rest_para()?;
                para_list.set_rest_para(rest_para);
                Ok(ASTNode::new(para_list, Span::new(begin, self.mark_end())))
            }
            TokenKind::Identifier | TokenKind::LeftBracket | TokenKind::LeftBrace => {
                loop {
                    let para = self.parse_para()?;
                    para_list.push_para(para);

                    if !self.kind_is(TokenKind::Comma) {
                        break;
                    }

                    self.forward();
                }

                if self.peek_kind() == TokenKind::Ellipsis {
                    para_list.set_rest_para(self.parse_rest_para()?);
                }

                Ok(ASTNode::new(para_list, Span::new(begin, self.mark_end())))
            }
            _ => Err(self.expect_error("ParaList", "Identifier or Ellipsis")),
        }
    }

    /*
    注意调用 type_annotation 不能吃掉 :, : 是该函数内部吃掉的
    */
    fn parse_type_annotation(&mut self) -> Result<ASTNode<TypeAnnotation>, ParserError> {
        let begin = self.mark_begin();
        self.eat(TokenKind::Colon)?;
        let type_ = self.parse_type()?;
        Ok(ASTNode::new(
            TypeAnnotation::new(type_),
            Span::new(begin, self.mark_end()),
        ))
    }

    fn parse_rest_para(&mut self) -> Result<ASTNode<RestPara>, ParserError> {
        Err(self.unsupported_error("rest parameter"))
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
        let begin = self.mark_begin();

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

        para.set_para_name(self.parse_identifier()?);

        if self.kind_is(TokenKind::QuestionMark) {
            para.set_question_mark();
            self.forward();
            if self.kind_is(TokenKind::Colon) {
                para.set_type_annotation(self.parse_type_annotation()?);
            }
        } else {
            if self.kind_is(TokenKind::Colon) {
                para.set_type_annotation(self.parse_type_annotation()?);
            }

            if self.kind_is(TokenKind::Assign) {
                para.set_initializer(self.parse_initializer()?);
            }
        }

        Ok(ASTNode::new(para, Span::new(begin, self.mark_end())))
    }

    fn parse_initializer(&mut self) -> Result<ASTNode<Initializer>, ParserError> {
        let begin = self.mark_begin();
        self.eat(TokenKind::Assign)?;
        let initializer = Initializer::new(self.parse_exp()?);
        Ok(ASTNode::new(initializer, Span::new(begin, self.mark_end())))
    }

    fn parse_type(&mut self) -> Result<ASTNode<Type>, ParserError> {
        let begin = self.mark_begin();

        if self.kind_is(TokenKind::LeftParen) {
            Ok(ASTNode::new(
                Type::FunctionType(self.parse_func_type()?),
                Span::new(begin, self.mark_end()),
            ))
        } else {
            Ok(ASTNode::new(
                Type::PrimaryType(self.parse_primary_type()?),
                Span::new(begin, self.mark_end()),
            ))
        }
    }

    /*
    primaryType:
        predefinedType ('[' ']')?					# PredefinedPrimType
        | typeReference	('[' ']')?			       	# ReferencePrimType
        | '[' tupleElementTypes ']'					# TuplePrimType
        | typeQuery                                 #QueryPrimType
        | objectType								# ObjectPrimType;
    */
    fn parse_primary_type(&mut self) -> Result<ASTNode<PrimaryType>, ParserError> {
        let begin = self.mark_begin();

        // [
        if self.kind_is(TokenKind::LeftBrace) {
            self.eat(TokenKind::LeftBrace)?;
            let tuple_type = PrimaryType::TupleType(self.parse_tuple_type()?);
            self.eat(TokenKind::RightBrace)?;
            return Ok(ASTNode::new(tuple_type, Span::new(begin, self.mark_end())));
        }

        // {
        if self.kind_is(TokenKind::LeftBracket) {
            return Ok(ASTNode::new(
                PrimaryType::ObjectType(self.parse_object_type()?),
                Span::new(begin, self.mark_end()),
            ));
        }

        /*
            typeQuery
                : 'typeof' typeQueryExpression
            typeQueryExpression:
                : (identifier '.')+ identifier
        ;
            */
        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Typeof)) {
            let type_query_begin = self.mark_begin();
            let mut type_query = TypeQuery::default();
            self.eat(TokenKind::KeyWord(KeyWordKind::Typeof))?;
            loop {
                type_query.push_type_path(self.parse_identifier()?);
                if !self.kind_is(TokenKind::Dot) {
                    break;
                }
                self.forward();
            }
            let type_query = ASTNode::new(type_query, Span::new(type_query_begin, self.mark_end()));

            return Ok(ASTNode::new(
                PrimaryType::TypeQuery(type_query),
                Span::new(begin, self.mark_end()),
            ));
        }

        if self.kind_is(TokenKind::Identifier) {
            let type_ref;

            if self.nextkind_is(TokenKind::Dot) {
                type_ref = TypeRef::new_namespace(self.parse_namespace_name()?);
            } else {
                type_ref = TypeRef::new_identifier(self.parse_identifier()?);
            }

            if self.kind_is(TokenKind::LeftBrace) {
                self.eat(TokenKind::LeftBrace)?;
                self.eat(TokenKind::RightBrace)?;

                let type_ref = ASTNode::new(type_ref, Span::new(begin, self.mark_end()));
                let array_type_ref = ASTNode::new(
                    ArrayTypeRef::new(type_ref),
                    Span::new(begin, self.mark_end()),
                );

                return Ok(ASTNode::new(
                    PrimaryType::ArrayTypeRef(array_type_ref),
                    Span::new(begin, self.mark_end()),
                ));
            } else {
                return Ok(ASTNode::new(
                    PrimaryType::TypeRef(ASTNode::new(type_ref, Span::new(begin, self.mark_end()))),
                    Span::new(begin, self.mark_end()),
                ));
            }
        }

        match self.peek_kind() {
            TokenKind::KeyWord(KeyWordKind::Any) => {
                self.forward();
                if self.kind_is(TokenKind::LeftBrace) {
                    self.eat(TokenKind::LeftBrace)?;
                    self.eat(TokenKind::RightBrace)?;

                    let any_type =
                        ASTNode::new(PredefinedType::Any, Span::new(begin, self.mark_end()));

                    Ok(ASTNode::new(
                        PrimaryType::ArrayPredefinedType(ASTNode::new(
                            ArrayPredefinedType::new(any_type),
                            Span::new(begin, self.mark_end()),
                        )),
                        Span::new(begin, self.mark_end()),
                    ))
                } else {
                    Ok(ASTNode::new(
                        PrimaryType::PredefinedType(ASTNode::new(
                            PredefinedType::Any,
                            Span::new(begin, self.mark_end()),
                        )),
                        Span::new(begin, self.mark_end()),
                    ))
                }
            }

            TokenKind::KeyWord(KeyWordKind::Number) => {
                self.forward();
                if self.kind_is(TokenKind::LeftBrace) {
                    self.eat(TokenKind::LeftBrace)?;
                    self.eat(TokenKind::RightBrace)?;

                    let number_type =
                        ASTNode::new(PredefinedType::Number, Span::new(begin, self.mark_end()));

                    Ok(ASTNode::new(
                        PrimaryType::ArrayPredefinedType(ASTNode::new(
                            ArrayPredefinedType::new(number_type),
                            Span::new(begin, self.mark_end()),
                        )),
                        Span::new(begin, self.mark_end()),
                    ))
                } else {
                    Ok(ASTNode::new(
                        PrimaryType::PredefinedType(ASTNode::new(
                            PredefinedType::Number,
                            Span::new(begin, self.mark_end()),
                        )),
                        Span::new(begin, self.mark_end()),
                    ))
                }
            }

            TokenKind::KeyWord(KeyWordKind::Boolean) => {
                self.forward();
                if self.kind_is(TokenKind::LeftBrace) {
                    self.eat(TokenKind::LeftBrace)?;
                    self.eat(TokenKind::RightBrace)?;

                    let boolean_type =
                        ASTNode::new(PredefinedType::Boolean, Span::new(begin, self.mark_end()));

                    Ok(ASTNode::new(
                        PrimaryType::ArrayPredefinedType(ASTNode::new(
                            ArrayPredefinedType::new(boolean_type),
                            Span::new(begin, self.mark_end()),
                        )),
                        Span::new(begin, self.mark_end()),
                    ))
                } else {
                    Ok(ASTNode::new(
                        PrimaryType::PredefinedType(ASTNode::new(
                            PredefinedType::Boolean,
                            Span::new(begin, self.mark_end()),
                        )),
                        Span::new(begin, self.mark_end()),
                    ))
                }
            }

            TokenKind::KeyWord(KeyWordKind::String) => {
                self.forward();
                if self.kind_is(TokenKind::LeftBrace) {
                    self.eat(TokenKind::LeftBrace)?;
                    self.eat(TokenKind::RightBrace)?;

                    let string_type =
                        ASTNode::new(PredefinedType::String, Span::new(begin, self.mark_end()));

                    Ok(ASTNode::new(
                        PrimaryType::ArrayPredefinedType(ASTNode::new(
                            ArrayPredefinedType::new(string_type),
                            Span::new(begin, self.mark_end()),
                        )),
                        Span::new(begin, self.mark_end()),
                    ))
                } else {
                    Ok(ASTNode::new(
                        PrimaryType::PredefinedType(ASTNode::new(
                            PredefinedType::String,
                            Span::new(begin, self.mark_end()),
                        )),
                        Span::new(begin, self.mark_end()),
                    ))
                }
            }

            TokenKind::KeyWord(KeyWordKind::Symbol) => {
                self.forward();
                if self.kind_is(TokenKind::LeftBrace) {
                    self.eat(TokenKind::LeftBrace)?;
                    self.eat(TokenKind::RightBrace)?;

                    let symbol_type =
                        ASTNode::new(PredefinedType::Symbol, Span::new(begin, self.mark_end()));

                    Ok(ASTNode::new(
                        PrimaryType::ArrayPredefinedType(ASTNode::new(
                            ArrayPredefinedType::new(symbol_type),
                            Span::new(begin, self.mark_end()),
                        )),
                        Span::new(begin, self.mark_end()),
                    ))
                } else {
                    Ok(ASTNode::new(
                        PrimaryType::PredefinedType(ASTNode::new(
                            PredefinedType::Symbol,
                            Span::new(begin, self.mark_end()),
                        )),
                        Span::new(begin, self.mark_end()),
                    ))
                }
            }

            TokenKind::KeyWord(KeyWordKind::Void) => {
                self.forward();
                if self.kind_is(TokenKind::LeftBrace) {
                    self.eat(TokenKind::LeftBrace)?;
                    self.eat(TokenKind::RightBrace)?;

                    let void_type =
                        ASTNode::new(PredefinedType::Void, Span::new(begin, self.mark_end()));

                    Ok(ASTNode::new(
                        PrimaryType::ArrayPredefinedType(ASTNode::new(
                            ArrayPredefinedType::new(void_type),
                            Span::new(begin, self.mark_end()),
                        )),
                        Span::new(begin, self.mark_end()),
                    ))
                } else {
                    Ok(ASTNode::new(
                        PrimaryType::PredefinedType(ASTNode::new(
                            PredefinedType::Void,
                            Span::new(begin, self.mark_end()),
                        )),
                        Span::new(begin, self.mark_end()),
                    ))
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
        let begin = self.mark_begin();

        let type_;
        let mut para_list = None;
        self.eat(TokenKind::LeftParen)?;
        if !self.kind_is(TokenKind::RightParen) {
            para_list = Some(self.parse_para_list()?);
        }
        self.eat(TokenKind::RightParen)?;
        self.eat(TokenKind::ARROW)?;
        type_ = self.parse_type()?;

        Ok(ASTNode::new(
            FunctionType::new(para_list, type_),
            Span::new(begin, self.mark_end()),
        ))
    }

    fn parse_decorators(&mut self) -> Result<ASTNode<Decorators>, ParserError> {
        Err(self.unsupported_error("decorators"))
    }

    fn parse_access_modifier(&mut self) -> Result<ASTNode<AccessModifier>, ParserError> {
        let begin = self.mark_begin();

        let access_modifier = match self.peek_kind() {
            TokenKind::KeyWord(KeyWordKind::Public) => AccessModifier::Public,
            TokenKind::KeyWord(KeyWordKind::Private) => AccessModifier::Private,
            TokenKind::KeyWord(KeyWordKind::Protected) => AccessModifier::Protected,
            _ => return Err(self.expect_error("Access Modifier", "public or protected or private")),
        };

        self.forward();
        Ok(ASTNode::new(
            access_modifier,
            Span::new(begin, self.mark_end()),
        ))
    }

    /*
    interfaceDeclaration:
    Export? Declare? Interface Identifier typeParameters? (
        Extends typeReference (',' typeReference)*
    )? objectType SemiColon?;
    */
    fn parse_interface_decl(&mut self) -> Result<ASTNode<InterfaceDecl>, ParserError> {
        let begin = self.mark_begin();

        let mut interface_decl = InterfaceDecl::default();
        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Export)) {
            interface_decl.set_export();
            self.forward();
        }
        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Declare)) {
            interface_decl.set_declare();
            self.forward();
        }
        self.eat(TokenKind::KeyWord(KeyWordKind::Interface))?;

        interface_decl.set_interface_name(self.parse_identifier()?);
        if self.kind_is(TokenKind::LessThan) {
            interface_decl.set_type_paras(self.parse_type_paras()?);
        }

        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Extends)) {
            let extends_begin = self.mark_begin();
            self.forward();
            loop {
                interface_decl.push_extends(ASTNode::new(
                    Extends::new(self.parse_type_ref()?),
                    Span::new(extends_begin, self.mark_end()),
                ));
                if !self.kind_is(TokenKind::Comma) {
                    break;
                }
                self.forward();
            }
        }

        interface_decl.set_object_type(self.parse_object_type()?);
        if self.kind_is(TokenKind::SemiColon) {
            self.forward();
        }

        Ok(ASTNode::new(
            interface_decl,
            Span::new(begin, self.mark_end()),
        ))
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
        let begin = self.mark_begin();

        let mut object_type = ObjectType::default();
        self.eat(TokenKind::LeftBracket)?;
        loop {
            if self.kind_is(TokenKind::RightBracket) {
                break;
            }

            let type_member = self.parse_type_member()?;
            object_type.push_type_member(type_member);
            match self.peek_kind() {
                TokenKind::Comma | TokenKind::SemiColon => {
                    self.forward();
                }

                _ => {
                    if !self.kind_is(TokenKind::RightBracket) && !self.is_new_line() {
                        return Err(self.expect_error("Object Type", ", or ; or }"));
                    }
                }
            }
        }
        self.eat(TokenKind::RightBracket)?;
        Ok(ASTNode::new(object_type, Span::new(begin, self.mark_end())))
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
        let begin = self.mark_begin();

        match self.peek_kind() {
            TokenKind::LeftParen | TokenKind::LessThan => {
                Ok(ASTNode::new(TypeMember::CallSig(self.parse_call_sig()?), Span::new(begin, self.mark_end())))
            }
            TokenKind::KeyWord(KeyWordKind::New) => {
                Ok(ASTNode::new(TypeMember::ConstructSig(self.parse_construct_sig()?), Span::new(begin, self.mark_end())))
            }
            TokenKind::LeftBrace => {
                Ok(ASTNode::new(TypeMember::IndexSig(self.parse_index_sig()?), Span::new(begin, self.mark_end())))
            }
            TokenKind::KeyWord(KeyWordKind::ReadOnly) => {
                Ok(ASTNode::new(TypeMember::PropertySig(self.parse_property_sig()?), Span::new(begin, self.mark_end())))
            }
            TokenKind::Identifier => {
                // attention: do not exchange the order of this if below
                // because property_sig can be the prefix of the method_sig
                if let Some(method_sig) = self.try_to(Parser::parse_method_sig) {
                    return Ok(ASTNode::new(TypeMember::MethodSig(method_sig), Span::new(begin, self.mark_end())));
                }

                if let Some(property_sig) = self.try_to(Parser::parse_property_sig) {
                    return Ok(ASTNode::new(TypeMember::PropertySig(property_sig), Span::new(begin, self.mark_end())));
                }

                Err(self.expect_error("typeMember", "propertySignature or methodSignature"))
            }
            _ => {
                Err(self.expect_error("typeMember", "propertySignature or callSignature or constructSignature or indexSignature or methodSignature"))
            }
        }
    }

    /*
    enumDeclaration:
        Const? Enum Identifier enumBody;
    */
    fn parse_enum_stat(&mut self) -> Result<ASTNode<EnumStat>, ParserError> {
        let begin = self.mark_begin();

        let mut enum_decl = EnumStat::default();
        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Const)) {
            enum_decl.set_const();
            self.forward();
        }

        self.eat(TokenKind::KeyWord(KeyWordKind::Enum))?;
        enum_decl.set_enum_name(self.parse_identifier()?);
        enum_decl.set_enum_body(self.parse_enum_body()?);

        Ok(ASTNode::new(enum_decl, Span::new(begin, self.mark_end())))
    }

    /*
    enumBody 内部可能没有元素
    enumBody: '{' (enumMember (',' enumMember)* ','?)? '}';
    */
    fn parse_enum_body(&mut self) -> Result<ASTNode<EnumBody>, ParserError> {
        let begin = self.mark_begin();

        let mut enum_body = EnumBody::default();
        self.eat(TokenKind::LeftBracket)?;
        loop {
            if self.kind_is(TokenKind::RightBracket) {
                break;
            }
            enum_body.push_enum_member(self.parse_enum_member()?);

            if self.kind_is(TokenKind::Comma) {
                self.forward();
            }
        }
        self.eat(TokenKind::RightBracket)?;
        Ok(ASTNode::new(enum_body, Span::new(begin, self.mark_end())))
    }

    /*
    enumMember: Identifier initializer?;
    */
    fn parse_enum_member(&mut self) -> Result<ASTNode<EnumMember>, ParserError> {
        let begin = self.mark_begin();
        let mut enum_member = EnumMember::default();
        enum_member.set_enum_member_name(self.parse_identifier()?);

        if self.kind_is(TokenKind::Assign) {
            enum_member.set_initializer(self.parse_initializer()?);
        }

        Ok(ASTNode::new(enum_member, Span::new(begin, self.mark_end())))
    }

    /*
    variableStatement:
    accessibilityModifier? varModifier? ReadOnly? variableDeclarationList SemiColon?
    | Declare varModifier? variableDeclarationList SemiColon?;
    */
    fn parse_var_stat(&mut self) -> Result<ASTNode<VarStat>, ParserError> {
        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Declare)) {
            return self.parse_var_stat2();
        } else {
            return self.parse_var_stat1();
        }
    }

    /*
    accessibilityModifier? varModifier? ReadOnly? variableDeclarationList SemiColon?
    */
    fn parse_var_stat1(&mut self) -> Result<ASTNode<VarStat>, ParserError> {
        let begin = self.mark_begin();

        let mut var_stat = VarStat::default();
        match self.peek_kind() {
            TokenKind::KeyWord(KeyWordKind::Public)
            | TokenKind::KeyWord(KeyWordKind::Protected)
            | TokenKind::KeyWord(KeyWordKind::Private) => {
                var_stat.set_access_modifier(self.parse_access_modifier()?);
            }
            _ => (),
        }

        match self.peek_kind() {
            TokenKind::KeyWord(KeyWordKind::Let)
            | TokenKind::KeyWord(KeyWordKind::Var)
            | TokenKind::KeyWord(KeyWordKind::Const) => {
                var_stat.set_var_modifier(self.parse_var_modifier()?);
            }
            _ => (),
        }

        if self.kind_is(TokenKind::KeyWord(KeyWordKind::ReadOnly)) {
            var_stat.set_readonly();
            self.forward();
        }

        var_stat.set_var_decl_list(self.parse_var_decl_list()?);

        if self.kind_is(TokenKind::SemiColon) {
            self.forward();
        }

        Ok(ASTNode::new(var_stat, Span::new(begin, self.mark_end())))
    }

    /*
    Declare varModifier? variableDeclarationList SemiColon?;
    */
    fn parse_var_stat2(&mut self) -> Result<ASTNode<VarStat>, ParserError> {
        let begin = self.mark_begin();

        let mut var_stat = VarStat::default();
        if self.kind_is(TokenKind::KeyWord(KeyWordKind::Declare)) {
            self.forward();
            var_stat.set_declare();
        }

        match self.peek_kind() {
            TokenKind::KeyWord(KeyWordKind::Let)
            | TokenKind::KeyWord(KeyWordKind::Var)
            | TokenKind::KeyWord(KeyWordKind::Const) => {
                var_stat.set_var_modifier(self.parse_var_modifier()?);
            }
            _ => (),
        }

        var_stat.set_var_decl_list(self.parse_var_decl_list()?);

        if self.kind_is(TokenKind::SemiColon) {
            self.forward();
        }

        Ok(ASTNode::new(var_stat, Span::new(begin, self.mark_end())))
    }

    fn parse_var_modifier(&mut self) -> Result<ASTNode<VarModifier>, ParserError> {
        let begin = self.mark_begin();

        let var_modifier = match self.peek_kind() {
            TokenKind::KeyWord(KeyWordKind::Let) => VarModifier::Let,
            TokenKind::KeyWord(KeyWordKind::Var) => VarModifier::Var,
            TokenKind::KeyWord(KeyWordKind::Const) => VarModifier::Const,
            _ => return Err(self.expect_error("For Var Statement", "Let or Var or Const")),
        };

        self.forward();
        Ok(ASTNode::new(
            var_modifier,
            Span::new(begin, self.mark_end()),
        ))
    }

    /*
    typeAliasDeclaration
        : 'type' Identifier typeParameters? '=' type_ SemiColon
    ;
    */
    fn parse_typealias_stat(&mut self) -> Result<ASTNode<TypeAlias>, ParserError> {
        let begin = self.mark_begin();
        let new_type;
        let mut type_paras = None;
        let type_;

        self.eat(TokenKind::KeyWord(KeyWordKind::Type))?;
        new_type = self.parse_identifier()?;
        if self.kind_is(TokenKind::LessThan) {
            type_paras = Some(self.parse_type_paras()?);
        }
        self.eat(TokenKind::Assign)?;
        type_ = self.parse_type()?;
        self.eat(TokenKind::SemiColon)?;

        let typealias = TypeAlias::new(new_type, type_paras, type_);
        Ok(ASTNode::new(typealias, Span::new(begin, self.mark_end())))
    }

    fn parse_identifier(&mut self) -> Result<ASTNode<Identifier>, ParserError> {
        let begin = self.mark_begin();
        Ok(ASTNode::new(
            Identifier::new(&self.extact_identifier()?),
            Span::new(begin, self.mark_end()),
        ))
    }
}
