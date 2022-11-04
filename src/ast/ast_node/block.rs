use crate::ast::{visulize::Visualizable, ASTNode};

use super::stat::Stat;

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

impl Visualizable for Block {
    fn draw(&self, id: usize) {
        todo!()
    }
}

pub struct CaseBlock {}
impl Visualizable for CaseBlock {
    fn draw(&self, id: usize) {
        todo!()
    }
}
