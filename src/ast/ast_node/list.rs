use crate::ast::{visulize::Visualizable, ASTNode, AstGraph};

use super::stat::Stat;

pub struct StatList {
    stats: Vec<ASTNode<Stat>>,
}

impl Visualizable for StatList {
    fn draw(&self, id: usize, graph: &mut AstGraph) {
        todo!()
    }
}
