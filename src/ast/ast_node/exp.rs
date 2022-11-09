use crate::ast::visulize::Visualizable;
use crate::ast::{ASTNode, AstGraph};

use super::literal::Literal;

#[derive(Visualizable, Default)]
pub struct ExpSeq {
    exps: Vec<ASTNode<Exp>>,
}
impl ExpSeq {
    pub(crate) fn push_exp(&mut self, single_exp: ASTNode<Exp>) {
        self.exps.push(single_exp);
    }
}

#[derive(Visualizable)]
pub enum Exp {
    Literal(ASTNode<Literal>),
}
