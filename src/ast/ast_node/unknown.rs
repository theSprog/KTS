use crate::{
    ast::{visulize::Visualizable, AstGraph, NodeInfo},
    compiler_internal_error,
};

pub struct Unknown {}

impl Unknown {
    pub fn new() -> Self {
        Self {}
    }
}

impl Visualizable for Unknown {
    fn draw(&self, self_info: NodeInfo, graph: &mut AstGraph) {
        compiler_internal_error!("Cannot draw unknown node, you must be wrong on something");
    }
}
