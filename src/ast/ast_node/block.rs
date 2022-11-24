use super::clause::*;
use super::stat::Stat;
use crate::ast::visulize::AstGraph;
use crate::ast::{visulize::Visualizable, ASTNode, NodeInfo};

#[derive(Visualizable, Default)]
pub struct Block {
    stats: Vec<ASTNode<Stat>>,
}
impl Block {
    pub(crate) fn push(&mut self, stat: ASTNode<Stat>) {
        self.stats.push(stat);
    }
}

#[derive(Visualizable, Default)]
pub struct CaseBlock {
    case_clauses: Option<ASTNode<CaseClauses>>,
    default_clause: Option<ASTNode<DefaultClause>>,
}
impl CaseBlock {
    pub(crate) fn set_case_clauses(&mut self, case_clauses: ASTNode<CaseClauses>) {
        self.case_clauses = Some(case_clauses);
    }

    pub(crate) fn set_default_clause(&mut self, default_clause: ASTNode<DefaultClause>) {
        self.default_clause = Some(default_clause);
    }
}
