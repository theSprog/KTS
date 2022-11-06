use super::stat::Stat;
use crate::ast::AstGraph;
use crate::ast::{visulize::Visualizable, ASTNode};

#[derive(Visualizable, Default)]
pub struct Block {
    stats: Vec<Box<ASTNode<Stat>>>,
}
impl Block {
    pub(crate) fn push(&mut self, stat: ASTNode<Stat>) {
        self.stats.push(Box::new(stat));
    }
}

pub struct CaseBlock {}
impl Visualizable for CaseBlock {
    fn draw(&self, id: usize, graph: &mut crate::ast::AstGraph) {
        todo!()
    }
}
