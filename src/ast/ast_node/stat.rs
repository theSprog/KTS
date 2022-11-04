use visulize::Vis;

use super::{
    block::Block,
    decl::*,
    exp::ExpSeq,
    literal::{Identifier, Value},
    unknown::Unknown,
};
use crate::{
    ast::{visulize::Visualizable, ASTNode, AST_GRAPH},
    lexer::token_kind::TokenKind,
};

// #[macro_export]
// macro_rules! unknown_stat {
//     () => {
//         Stat::Unknown
//     };
// }

pub enum Stat {
    ImportStat(ASTNode<ImportStat>),
    ExportStat(ASTNode<ExportStat>),
    EmptyStat(ASTNode<EmptyStat>),
    Block(ASTNode<Block>),
    FuncDecl(ASTNode<FuncDecl>),
    FuncExpDecl(ASTNode<FuncExpDecl>),
    GenFuncDecl(ASTNode<GenFuncDecl>),
    Unknown(ASTNode<Unknown>),
}

impl Default for Stat {
    fn default() -> Self {
        Stat::Unknown(ASTNode::new(Unknown::new()))
    }
}

impl Visualizable for Stat {
    fn draw(&self, id: usize) {
        AST_GRAPH::put_node(id, "Stat");

        match self {
            Stat::ImportStat(import_stat) => {
                import_stat.draw();
                AST_GRAPH::put_edge(id, import_stat.id);
            }
            Stat::ExportStat(export_stat) => todo!(),
            Stat::EmptyStat(empty_stat) => todo!(),
            Stat::Block(block) => todo!(),
            Stat::FuncDecl(func_decl) => {
                func_decl.draw();
                AST_GRAPH::put_edge(id, func_decl.id);
            }
            Stat::FuncExpDecl(func_exp_decl) => todo!(),
            Stat::GenFuncDecl(gen_func_decl) => todo!(),

            Stat::Unknown(unknow) => {
                unknow.draw();
                AST_GRAPH::put_edge(id, unknow.id);
            }
        }
    }
}

pub struct ImportStat {
    all: Option<ASTNode<TokenKind>>,     // *
    alias: Option<ASTNode<Identifier>>,  // alias of *
    import: Option<ASTNode<Identifier>>, // import can not be alias

    imports: Vec<ASTNode<ImportAlias>>, // {a as b, c as d, ...}
    from: ASTNode<Value>,               // String literal
}

impl ImportStat {
    pub fn new() -> Self {
        ImportStat {
            all: None,
            alias: None,
            import: Default::default(),
            imports: Vec::new(),
            from: Default::default(),
        }
    }

    pub(crate) fn set_all(&mut self) {
        self.all = Some(ASTNode::new(TokenKind::Multiply));
    }

    pub(crate) fn set_all_alias(&mut self, context: &str) {
        self.alias = Some(ASTNode::new(Identifier::new(context)));
    }

    pub(crate) fn set_import(&mut self, import: &str) {
        self.import = Some(ASTNode::new(Identifier::new(import)));
    }

    pub(crate) fn push_import(&mut self, import: &str, alias: Option<&str>) {
        self.imports
            .push(ASTNode::new(ImportAlias::new(import, alias)));
    }

    pub(crate) fn set_from(&mut self, alias: &str) {
        self.from = ASTNode::new(Value::String(String::from(alias)));
    }
}

impl Visualizable for ImportStat {
    fn draw(&self, id: usize) {
        AST_GRAPH::put_node(id, "ImportStat");

        self.all.draw(id);
        self.alias.draw(id);
        self.import.draw(id);
        self.imports.draw(id);

        self.from.draw();
        AST_GRAPH::put_edge(id, self.from.id);
    }
}

pub struct ImportAlias {
    import: ASTNode<Identifier>,
    alias: Option<ASTNode<Identifier>>,
}

impl ImportAlias {
    fn new(import: &str, alias: Option<&str>) -> Self {
        let alias = match alias {
            Some(alias) => Some(ASTNode::new(Identifier::new(alias))),
            None => None,
        };

        Self {
            import: ASTNode::new(Identifier::new(import)),
            alias,
        }
    }
}

impl Visualizable for ImportAlias {
    fn draw(&self, id: usize) {
        AST_GRAPH::put_node(id, "ImportAlias");

        self.import.draw();
        AST_GRAPH::put_edge(id, self.import.id);

        self.alias.draw(id);
    }
}

pub struct ExportStat {}
impl Visualizable for ExportStat {
    fn draw(&self, id: usize) {
        todo!()
    }
}
pub struct EmptyStat {}
impl Visualizable for EmptyStat {
    fn draw(&self, id: usize) {
        todo!()
    }
}
pub struct IfStat {
    exp: ASTNode<ExpSeq>,
    stat: ASTNode<Stat>,
    else_stat: Option<ASTNode<Stat>>,
}
impl Visualizable for IfStat {
    fn draw(&self, id: usize) {
        todo!()
    }
}
// pub struct IterStat {}
// impl Visualizable for IterStat {}
// pub struct ContinueStat {}
// impl Visualizable for ContinueStat {}
// pub struct BreakStat {}
// impl Visualizable for BreakStat {}
// pub struct ReturnStat {}
// impl Visualizable for ReturnStat {}
// pub struct YieldStat {}
// impl Visualizable for YieldStat {}
// pub struct WithStat {}
// impl Visualizable for WithStat {}
// pub struct LabelledStat {}
// impl Visualizable for LabelledStat {}
// pub struct SwitchStat {}
// impl Visualizable for SwitchStat {}
// pub struct ThrowStat {}
// impl Visualizable for ThrowStat {}
// pub struct TryStat {}
// impl Visualizable for TryStat {}
// pub struct DebuggerStat {}
// impl Visualizable for DebuggerStat {}
