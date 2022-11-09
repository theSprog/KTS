use crate::ast::AstGraph;
use visulize::Visualizable;

use super::{
    block::Block, decl::*, exp::ExpSeq, identifier::Identifier, literal::Literal, unknown::Unknown,
};
use crate::{
    ast::{visulize::Visualizable, ASTNode},
    lexer::token_kind::{KeyWordKind, TokenKind},
};

pub struct StatList {
    stats: Vec<ASTNode<Stat>>,
}

impl Visualizable for StatList {
    fn draw(&self, id: usize, graph: &mut AstGraph) {
        todo!()
    }
}

#[derive(Visualizable)]
pub enum Stat {
    ImportStat(ASTNode<ImportStat>),
    ExportStat(ASTNode<ExportStat>),
    EmptyStat(ASTNode<EmptyStat>),
    Block(ASTNode<Block>),
    ClassDecl(ASTNode<ClassDecl>),
    InterfaceDecl(ASTNode<InterfaceDecl>),
    AbsDecl(ASTNode<AbsDecl>),
    NamespaceDecl(ASTNode<NamespaceDecl>),
    IfStat(ASTNode<IfStat>),
    IterStat(ASTNode<IterStat>),

    FuncDecl(ASTNode<FuncDecl>),
    FuncExpDecl(ASTNode<FuncExpDecl>),
    GenFuncDecl(ASTNode<GenFuncDecl>),
    Unknown(ASTNode<Unknown>),
}

impl Default for Stat {
    fn default() -> Self {
        Stat::Unknown(ASTNode::dummy())
    }
}

#[derive(Visualizable)]
pub struct ImportStat {
    import_kw: ASTNode<KeyWordKind>,
    from_block: ASTNode<FromBlock>,
}

impl Default for ImportStat {
    fn default() -> Self {
        ImportStat {
            import_kw: ASTNode::new(KeyWordKind::Import),
            from_block: Default::default(),
        }
    }
}

impl ImportStat {
    pub(crate) fn set_from_block(&mut self, from_block: ASTNode<FromBlock>) {
        self.from_block = from_block
    }
}

#[derive(Visualizable)]
pub struct FromBlock {
    all: Option<ASTNode<TokenKind>>,        // *
    alias: Option<ASTNode<Identifier>>,     // alias of *
    imported: Option<ASTNode<Identifier>>,  // imported can not be alias
    importeds: Vec<ASTNode<ImportedAlias>>, // {a as b, c as d, ...}
    from_value: ASTNode<Literal>,
}

impl Default for FromBlock {
    fn default() -> Self {
        FromBlock {
            all: None,
            alias: None,
            imported: None,
            importeds: Default::default(),
            from_value: Default::default(),
        }
    }
}

impl FromBlock {
    pub(crate) fn set_all(&mut self) {
        self.all = Some(ASTNode::new(TokenKind::Multiply));
    }

    pub(crate) fn set_all_alias(&mut self, context: &str) {
        self.alias = Some(ASTNode::new(Identifier::new(context)));
    }

    pub(crate) fn set_imported(&mut self, imported: &str) {
        self.imported = Some(ASTNode::new(Identifier::new(imported)));
    }

    pub(crate) fn push_imported_alias(&mut self, imported: &str, alias: Option<&str>) {
        self.importeds
            .push(ASTNode::new(ImportedAlias::new(imported, alias)));
    }

    pub(crate) fn set_from_value(&mut self, from_value: &str) {
        self.from_value = ASTNode::new(Literal::String(String::from(from_value)));
    }
}

#[derive(Visualizable)]
pub struct ImportedAlias {
    imported: ASTNode<Identifier>,
    alias: Option<ASTNode<Identifier>>,
}

impl ImportedAlias {
    fn new(imported: &str, alias: Option<&str>) -> Self {
        let alias = match alias {
            Some(alias) => Some(ASTNode::new(Identifier::new(alias))),
            None => None,
        };

        let imported = ASTNode::new(Identifier::new(imported));

        Self { imported, alias }
    }
}

#[derive(Visualizable)]
pub struct ExportStat {
    export_kw: ASTNode<KeyWordKind>,
    default: Option<ASTNode<KeyWordKind>>, // default keyword
    from_block: Option<ASTNode<FromBlock>>,
    stat: Option<Box<ASTNode<Stat>>>,
}

impl Default for ExportStat {
    fn default() -> Self {
        ExportStat {
            export_kw: ASTNode::new(KeyWordKind::Export),
            default: None,
            from_block: None,
            stat: None,
        }
    }
}

impl ExportStat {
    pub(crate) fn set_default(&mut self) {
        self.default = Some(ASTNode::new(KeyWordKind::Default));
    }

    pub(crate) fn set_from_block(&mut self, from_block: ASTNode<FromBlock>) {
        self.from_block = Some(from_block);
    }

    pub(crate) fn set_stat(&mut self, stat: ASTNode<Stat>) {
        self.stat = Some(Box::new(stat));
    }
}
#[derive(Visualizable)]
pub struct EmptyStat {}
impl EmptyStat {
    pub(crate) fn new() -> EmptyStat {
        Self {}
    }
}

#[derive(Visualizable, Default)]
pub struct IfStat {
    exp_seq: ASTNode<ExpSeq>,
    stat: Box<ASTNode<Stat>>,
    else_stat: Option<Box<ASTNode<Stat>>>,
}
impl IfStat {
    pub(crate) fn set_exp_seq(&mut self, exp_seq: ASTNode<ExpSeq>) {
        self.exp_seq = exp_seq;
    }

    pub(crate) fn set_stat(&mut self, stat: ASTNode<Stat>) {
        self.stat = Box::new(stat);
    }

    pub(crate) fn set_else_stat(&mut self, else_stat: ASTNode<Stat>) {
        self.else_stat = Some(Box::new(else_stat));
    }
}
#[derive(Visualizable)]
pub enum IterStat {
    DoStat(ASTNode<DoStat>),
    WhileStat(ASTNode<WhileStat>),
    ForVarStat(ASTNode<ForVarStat>),
    ForInStat(ASTNode<ForInStat>),
}

#[derive(Visualizable)]
pub struct DoStat {}

#[derive(Visualizable)]
pub struct WhileStat {}

#[derive(Visualizable)]
pub struct ForVarStat {}
#[derive(Visualizable)]
pub struct ForInStat {}

#[derive(Visualizable)]
pub struct ContinueStat {}

#[derive(Visualizable)]
pub struct BreakStat {}

#[derive(Visualizable)]
pub struct ReturnStat {}

#[derive(Visualizable)]
pub struct YieldStat {}

#[derive(Visualizable)]
pub struct WithStat {}

#[derive(Visualizable)]
pub struct LabelledStat {}

#[derive(Visualizable)]
pub struct SwitchStat {}

#[derive(Visualizable)]
pub struct ThrowStat {}

#[derive(Visualizable)]
pub struct TryStat {}

#[derive(Visualizable)]
pub struct DebuggerStat {}
