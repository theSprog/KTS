use crate::ast::visulize::AstGraph;
use crate::ast::visulize::Visualizable;
use crate::ast::{ASTNode, NodeInfo};

use super::exp::Exp;
use super::source_element::SourceElements;

#[derive(Visualizable, Default)]
pub struct CaseClauses {
    case_clauses: Vec<ASTNode<CaseClause>>,
}
impl CaseClauses {
    pub(crate) fn push_case_clause(&mut self, case_clause: ASTNode<CaseClause>) {
        self.case_clauses.push(case_clause);
    }
}

#[derive(Visualizable)]
pub struct CaseClause {
    exp: ASTNode<Exp>,
    stats: Option<ASTNode<SourceElements>>,
}
impl CaseClause {
    pub(crate) fn new(exp: ASTNode<Exp>, stats: Option<ASTNode<SourceElements>>) -> Self {
        Self { exp, stats }
    }
}

#[derive(Visualizable, Default)]
pub struct DefaultClause {
    stats: Option<ASTNode<SourceElements>>,
}
impl DefaultClause {
    pub(crate) fn new(stats: Option<ASTNode<SourceElements>>) -> Self {
        Self { stats }
    }
}
