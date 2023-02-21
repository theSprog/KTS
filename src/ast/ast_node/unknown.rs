use crate::ast::visulize::AstGraph;
use crate::ast::{visulize::Visualizable, NodeInfo};

pub struct Unknown {}

impl Unknown {
    pub fn new() -> Self {
        Self {}
    }
}

impl Visualizable for Unknown {
    fn draw(&self, self_info: NodeInfo, graph: &mut AstGraph) {
        unreachable!()
    }
}
