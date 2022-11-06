use crate::{ast::{visulize::Visualizable, AstGraph}, compiler_internal_error};

pub struct Unknown {}

impl Unknown {
    pub fn new() -> Self {
        Unknown {}
    }
}

impl Visualizable for Unknown {
    fn draw(&self, id: usize, graph: &mut AstGraph) {
        compiler_internal_error!("Cannot draw unknown node, you must be wrong on something");
    }
}
