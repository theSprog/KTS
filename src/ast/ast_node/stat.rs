use crate::ast::AstGraph;
use visulize::Visualizable;

use super::{
    block::{Block, CaseBlock},
    class::AccessModifier,
    decl::*,
    exp::{Exp, ExpSeq},
    identifier::Identifier,
    literal::Literal,
    parameter::{Initializer, TypeAnnotation},
    type_::TypeAlias,
    unknown::Unknown,
};
use crate::{
    ast::{visulize::Visualizable, ASTNode, NodeInfo},
    lexer::token_kind::{KeyWordKind, TokenKind},
};

#[derive(Visualizable)]
pub enum Stat {
    ImportStat(ImportStat),
    ExportStat(ExportStat),
    EmptyStat(EmptyStat),
    Block(Block),
    ClassDecl(ClassDecl),
    InterfaceDecl(InterfaceDecl),
    AbsDecl(AbsDecl),
    NamespaceDecl(NamespaceDecl),

    FuncDecl(FuncDecl),

    VarStat(VarStat),
    EnumStat(EnumStat),

    IfStat(IfStat),
    IterStat(IterStat),

    ContinueStat(ContinueStat),
    BreakStat(BreakStat),
    ReturnStat(ReturnStat),
    YieldStat(YieldStat),
    WithStat(WithStat),

    SwitchStat(SwitchStat),
    ThrowStat(ThrowStat),

    TypeAliasStat(TypeAlias),

    DebuggerStat(DebuggerStat),
    TryStat(TryStat),

    FuncExpDecl(FuncExpDecl),
    GenFuncDecl(GenFuncDecl),
    LabelledStat(LabelledStat),

    ExpStat(ExpSeq),

    Unknown(Unknown),
}

impl Default for Stat {
    fn default() -> Self {
        Stat::Unknown(Unknown::new())
    }
}

#[derive(Visualizable)]
pub struct ImportStat {
    import_block: ImportBlock,
}
impl ImportStat {
    pub(crate) fn new(import_block: ImportBlock) -> Self {
        Self { import_block }
    }
}

#[derive(Visualizable)]
pub enum ImportBlock {
    FromBlock(ASTNode<FromBlock>),
    ImportAssign(ASTNode<ImportAssign>),
}

#[derive(Visualizable)]
pub struct ImportAssign {
    identifier: ASTNode<Identifier>,
    namespace_name: ASTNode<NamespaceName>,
}
impl ImportAssign {
    pub(crate) fn new(
        identifier: ASTNode<Identifier>,
        namespace_name: ASTNode<NamespaceName>,
    ) -> Self {
        Self {
            identifier,
            namespace_name,
        }
    }
}

#[derive(Visualizable, Default)]
pub struct FromBlock {
    all: Option<TokenKind>,                // *
    alias: Option<ASTNode<Identifier>>,    // alias of *
    imported: Option<ASTNode<Identifier>>, // imported can not be alias
    importeds: Vec<ASTNode<PortedAlias>>,  // {a as b, c as d, ...}
    from_value: ASTNode<Literal>,
}

impl FromBlock {
    pub(crate) fn set_all(&mut self) {
        self.all = Some(TokenKind::Multiply);
    }

    pub(crate) fn set_all_alias(&mut self, context: ASTNode<Identifier>) {
        self.alias = Some(context);
    }

    pub(crate) fn set_imported(&mut self, imported: ASTNode<Identifier>) {
        self.imported = Some(imported);
    }

    pub(crate) fn push_imported_alias(&mut self, imported_alias: ASTNode<PortedAlias>) {
        self.importeds.push(imported_alias);
    }

    pub(crate) fn set_from_value(&mut self, from_value: ASTNode<Literal>) {
        self.from_value = from_value;
        //  Literal::String(String::from(from_value));
    }
}

#[derive(Visualizable)]
pub struct PortedAlias {
    ported: ASTNode<Identifier>,
    alias: Option<ASTNode<Identifier>>,
}

impl PortedAlias {
    pub(crate) fn new(ported: ASTNode<Identifier>, alias: Option<ASTNode<Identifier>>) -> Self {
        Self { ported, alias }
    }
}

#[derive(Visualizable, Default)]
pub struct ExportStat {
    default: Option<KeyWordKind>, // default keyword
    from_block: Option<ASTNode<FromBlock>>,
    stat: Option<ASTNode<Stat>>,
}

impl ExportStat {
    pub(crate) fn set_default(&mut self) {
        self.default = Some(KeyWordKind::Default);
    }

    pub(crate) fn set_from_block(&mut self, from_block: ASTNode<FromBlock>) {
        assert!(self.from_block.is_none());
        assert!(self.stat.is_none());

        self.from_block = Some(from_block);
    }

    pub(crate) fn set_stat(&mut self, stat: ASTNode<Stat>) {
        assert!(self.from_block.is_none());
        assert!(self.stat.is_none());

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
    declare: Option<KeyWordKind>,
    var_modifier: Option<ASTNode<VarModifier>>,
    readonly: Option<KeyWordKind>,
    var_decl_list: ASTNode<VarDeclList>,
}
impl VarStat {
    pub(crate) fn set_access_modifier(&mut self, access_modifier: ASTNode<AccessModifier>) {
        self.access_modifier = Some(access_modifier);
    }

    pub(crate) fn set_declare(&mut self) {
        self.declare = Some(KeyWordKind::Declare);
    }

    pub(crate) fn set_var_modifier(&mut self, var_modifier: ASTNode<VarModifier>) {
        self.var_modifier = Some(var_modifier);
    }

    pub(crate) fn set_readonly(&mut self) {
        self.readonly = Some(KeyWordKind::ReadOnly);
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
    fn draw(&self, self_info: NodeInfo, graph: &mut AstGraph) {
        match self {
            VarModifier::Let => graph.put_node(self_info, "let"),
            VarModifier::Const => graph.put_node(self_info, "const"),
            VarModifier::Var => graph.put_node(self_info, "var"),
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
    pub(crate) fn new(var_name: ASTNode<Identifier>) -> Self {
        Self {
            var_name,
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
    pub(crate) fn set_identifier(&mut self, identifier: ASTNode<Identifier>) {
        self.identifier = Some(identifier);
    }
}

#[derive(Visualizable, Default)]
pub struct BreakStat {
    identifier: Option<ASTNode<Identifier>>,
}
impl BreakStat {
    pub(crate) fn set_identifier(&mut self, identifier: ASTNode<Identifier>) {
        self.identifier = Some(identifier);
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
    pub(crate) fn new(identifier: ASTNode<Identifier>, stat: ASTNode<Stat>) -> Self {
        Self { identifier, stat }
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

#[derive(Visualizable, Default)]
pub struct EnumStat {
    const_: Option<KeyWordKind>,
    enum_name: ASTNode<Identifier>,
    enum_body: ASTNode<EnumBody>,
}
impl EnumStat {
    pub(crate) fn set_const(&mut self) {
        self.const_ = Some(KeyWordKind::Const)
    }

    pub(crate) fn set_enum_name(&mut self, enum_name: ASTNode<Identifier>) {
        self.enum_name = enum_name;
    }

    pub(crate) fn set_enum_body(&mut self, enum_body: ASTNode<EnumBody>) {
        self.enum_body = enum_body;
    }
}

#[derive(Visualizable, Default)]
pub struct EnumBody {
    enum_members: Vec<ASTNode<EnumMember>>,
}
impl EnumBody {
    pub(crate) fn push_enum_member(&mut self, enum_member: ASTNode<EnumMember>) {
        self.enum_members.push(enum_member);
    }
}

#[derive(Visualizable, Default)]
pub struct EnumMember {
    enum_member_name: ASTNode<Identifier>,
    initializer: Option<ASTNode<Initializer>>,
}
impl EnumMember {
    pub(crate) fn set_enum_member_name(&mut self, enum_member_name: ASTNode<Identifier>) {
        self.enum_member_name = enum_member_name;
    }

    pub(crate) fn set_initializer(&mut self, initializer: ASTNode<Initializer>) {
        self.initializer = Some(initializer);
    }
}

impl TryStat {}

#[derive(Visualizable)]
pub struct DebuggerStat {}
