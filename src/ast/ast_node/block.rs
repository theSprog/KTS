use super::stat::Stat;
use crate::ast::AST_GRAPH;
use crate::ast::{visulize::Visualizable, ASTNode};

#[derive(Visualizable)]
pub struct Block {
    stats: Vec<ASTNode<Stat>>,
}
impl Block {
    pub(crate) fn new() -> Self {
        Self { stats: Vec::new() }
    }

    pub(crate) fn push(&mut self, stat: ASTNode<Stat>) {
        self.stats.push(stat);
    }
}

pub struct CaseBlock {}
impl Visualizable for CaseBlock {
    fn draw(&self, id: usize) {
        todo!()
    }
}
