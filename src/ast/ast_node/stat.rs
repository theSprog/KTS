use crate::ast::AstGraph;
use visulize::Visualizable;

use super::{
    block::{Block, CaseBlock},
    class::AccessModifier,
    decl::*,
    exp::{Exp, ExpSeq},
    identifier::Identifier,
    literal::Literal,
    parameter::TypeAnnotation,
    unknown::Unknown,
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

    VarStat(ASTNode<VarStat>),

    IfStat(ASTNode<IfStat>),
    IterStat(ASTNode<IterStat>),

    ContinueStat(ASTNode<ContinueStat>),
    BreakStat(ASTNode<BreakStat>),
    ReturnStat(ASTNode<ReturnStat>),
    YieldStat(ASTNode<YieldStat>),
    WithStat(ASTNode<WithStat>),

    SwitchStat(ASTNode<SwitchStat>),
    ThrowStat(ASTNode<ThrowStat>),

    DebuggerStat(ASTNode<DebuggerStat>),
    TryStat(ASTNode<TryStat>),

    FuncExpDecl(ASTNode<FuncExpDecl>),
    GenFuncDecl(ASTNode<GenFuncDecl>),

    ExpStat(ASTNode<ExpSeq>),

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
    stat: Option<ASTNode<Stat>>,
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
        self.stat = Some(stat);
    }
}
#[derive(Visualizable)]
pub struct EmptyStat {}
impl EmptyStat {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

#[derive(Visualizable, Default)]
pub struct VarStat {
    access_modifier: Option<ASTNode<AccessModifier>>,
    declare: Option<ASTNode<KeyWordKind>>,
    var_modifier: Option<ASTNode<VarModifier>>,
    readonly: Option<ASTNode<KeyWordKind>>,
    var_decl_list: ASTNode<VarDeclList>,
}
impl VarStat {
    pub(crate) fn set_access_modifier(&mut self, access_modifier: ASTNode<AccessModifier>) {
        self.access_modifier = Some(access_modifier);
    }

    pub(crate) fn set_declare(&mut self) {
        self.declare = Some(ASTNode::new(KeyWordKind::Declare));
    }

    pub(crate) fn set_var_modifier(&mut self, var_modifier: ASTNode<VarModifier>) {
        self.var_modifier = Some(var_modifier);
    }

    pub(crate) fn set_readonly(&mut self) {
        self.readonly = Some(ASTNode::new(KeyWordKind::ReadOnly));
    }

    pub(crate) fn set_var_decl_list(&mut self, var_decl_list: ASTNode<VarDeclList>) {
        self.var_decl_list = var_decl_list;
    }
}

#[derive(Visualizable, Default)]
pub struct IfStat {
    exp_seq: ASTNode<ExpSeq>,
    stat: ASTNode<Stat>,
    else_stat: Option<ASTNode<Stat>>,
}
impl IfStat {
    pub(crate) fn set_exp_seq(&mut self, exp_seq: ASTNode<ExpSeq>) {
        self.exp_seq = exp_seq;
    }

    pub(crate) fn set_stat(&mut self, stat: ASTNode<Stat>) {
        self.stat = stat;
    }

    pub(crate) fn set_else_stat(&mut self, else_stat: ASTNode<Stat>) {
        self.else_stat = Some(else_stat);
    }
}
#[derive(Visualizable)]
pub enum IterStat {
    DoStat(ASTNode<DoStat>),
    WhileStat(ASTNode<WhileStat>),
    ForStat(ASTNode<ForStat>),
    ForVarStat(ASTNode<ForVarStat>),
    ForInStat(ASTNode<ForInStat>),
}

#[derive(Visualizable)]
pub struct DoStat {
    stat: ASTNode<Stat>,
    exp: ASTNode<Exp>,
}
impl DoStat {
    pub(crate) fn new(stat: ASTNode<Stat>, exp: ASTNode<Exp>) -> Self {
        Self { stat, exp }
    }
}

#[derive(Visualizable)]
pub struct WhileStat {
    exp: ASTNode<Exp>,
    stat: ASTNode<Stat>,
}
impl WhileStat {
    pub(crate) fn new(exp: ASTNode<Exp>, stat: ASTNode<Stat>) -> Self {
        Self { exp, stat }
    }
}

#[derive(Visualizable, Default)]
pub struct ForStat {
    init: Option<ASTNode<ExpSeq>>,
    cond: Option<ASTNode<Exp>>,
    action: Option<ASTNode<ExpSeq>>,
    stat: ASTNode<Stat>,
}
impl ForStat {
    pub(crate) fn set_init(&mut self, init: ASTNode<ExpSeq>) {
        self.init = Some(init);
    }

    pub(crate) fn set_cond(&mut self, cond: ASTNode<Exp>) {
        self.cond = Some(cond);
    }

    pub(crate) fn set_action(&mut self, action: ASTNode<ExpSeq>) {
        self.action = Some(action);
    }

    pub(crate) fn set_stat(&mut self, stat: ASTNode<Stat>) {
        self.stat = stat;
    }
}

#[derive(Visualizable)]
pub struct ForVarStat {
    var_modifier: ASTNode<VarModifier>,
    var_decl_list: ASTNode<VarDeclList>,
    cond: Option<ASTNode<Exp>>,
    action: Option<ASTNode<ExpSeq>>,
    stat: ASTNode<Stat>,
}
impl ForVarStat {
    pub(crate) fn new(
        var_modifier: ASTNode<VarModifier>,
        var_decl_list: ASTNode<VarDeclList>,
        cond: Option<ASTNode<Exp>>,
        action: Option<ASTNode<ExpSeq>>,
        stat: ASTNode<Stat>,
    ) -> ForVarStat {
        Self {
            var_modifier,
            var_decl_list,
            cond,
            action,
            stat,
        }
    }
}
#[derive(Visualizable)]
pub struct ForInStat {
    var: ASTNode<Exp>,
    exp: ASTNode<Exp>,
    stat: ASTNode<Stat>,
}
impl ForInStat {
    pub(crate) fn new(var: ASTNode<Exp>, exp: ASTNode<Exp>, stat: ASTNode<Stat>) -> ForInStat {
        Self { var, exp, stat }
    }
}

pub enum VarModifier {
    Let,
    Const,
    Var,
}
impl Visualizable for VarModifier {
    fn draw(&self, self_id: usize, graph: &mut AstGraph) {
        match self {
            VarModifier::Let => graph.put_node(self_id, "let"),
            VarModifier::Const => graph.put_node(self_id, "const"),
            VarModifier::Var => graph.put_node(self_id, "var"),
        }
    }
}

#[derive(Visualizable, Default)]
pub struct VarDeclList {
    var_decls: Vec<ASTNode<VarDecl>>,
}
impl VarDeclList {
    pub(crate) fn push_var_decl(&mut self, var_decl: ASTNode<VarDecl>) {
        self.var_decls.push(var_decl);
    }
}

#[derive(Visualizable)]
pub struct VarDecl {
    var_name: ASTNode<Identifier>,
    type_annotation: Option<ASTNode<TypeAnnotation>>,
    initializer: Option<ASTNode<Exp>>,
}
impl VarDecl {
    pub(crate) fn new(var_name: &str) -> Self {
        Self {
            var_name: ASTNode::new(Identifier::new(var_name)),
            type_annotation: None,
            initializer: None,
        }
    }

    pub(crate) fn set_type_annotation(&mut self, type_annotation: ASTNode<TypeAnnotation>) {
        self.type_annotation = Some(type_annotation);
    }

    pub(crate) fn set_initializer(&mut self, initializer: ASTNode<Exp>) {
        self.initializer = Some(initializer);
    }
}

#[derive(Visualizable, Default)]
pub struct ContinueStat {
    identifier: Option<ASTNode<Identifier>>,
}
impl ContinueStat {
    pub(crate) fn set_identifier(&mut self, identifier: &str) {
        self.identifier = Some(ASTNode::new(Identifier::new(identifier)));
    }
}

#[derive(Visualizable, Default)]
pub struct BreakStat {
    identifier: Option<ASTNode<Identifier>>,
}
impl BreakStat {
    pub(crate) fn set_identifier(&mut self, identifier: &str) {
        self.identifier = Some(ASTNode::new(Identifier::new(identifier)));
    }
}

#[derive(Visualizable, Default)]
pub struct ReturnStat {
    exp_seq: Option<ASTNode<ExpSeq>>,
}
impl ReturnStat {
    pub(crate) fn set_exp_seq(&mut self, exp_seq: ASTNode<ExpSeq>) {
        self.exp_seq = Some(exp_seq);
    }
}

#[derive(Visualizable, Default)]
pub struct YieldStat {
    exp_seq: Option<ASTNode<ExpSeq>>,
}
impl YieldStat {
    pub(crate) fn set_exp_seq(&mut self, exp_seq: ASTNode<ExpSeq>) {
        self.exp_seq = Some(exp_seq);
    }
}

#[derive(Visualizable, Default)]
pub struct WithStat {
    exp_seq: ASTNode<ExpSeq>,
    stat: ASTNode<Stat>,
}
impl WithStat {
    pub(crate) fn new(exp_seq: ASTNode<ExpSeq>, stat: ASTNode<Stat>) -> Self {
        Self { exp_seq, stat }
    }
}

#[derive(Visualizable)]
pub struct LabelledStat {
    identifier: ASTNode<Identifier>,
    stat: ASTNode<Stat>,
}
impl LabelledStat {
    pub(crate) fn new(identifier: &str, stat: ASTNode<Stat>) -> Self {
        Self {
            identifier: ASTNode::new(Identifier::new(identifier)),
            stat,
        }
    }
}

#[derive(Visualizable)]
pub struct SwitchStat {
    exp: ASTNode<Exp>,
    cases_block: ASTNode<CaseBlock>,
}
impl SwitchStat {
    pub(crate) fn new(exp: ASTNode<Exp>, cases_block: ASTNode<CaseBlock>) -> Self {
        Self { exp, cases_block }
    }
}

#[derive(Visualizable)]
pub struct ThrowStat {}

#[derive(Visualizable, Default)]
pub struct TryStat {
    block: ASTNode<Block>,
}
impl TryStat {
    pub(crate) fn set_block(&mut self, block: ASTNode<Block>) {
        self.block = block;
    }
}

#[derive(Visualizable)]
pub struct DebuggerStat {}
