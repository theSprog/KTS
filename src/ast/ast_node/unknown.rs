use crate::ast::{visulize::Visualizable, AST_GRAPH};

pub struct Unknown {}

impl Unknown {
    pub fn new() -> Self {
        Unknown {}
    }
}

impl Visualizable for Unknown {
    fn draw(&self, id: usize) {
        AST_GRAPH::put_node(id, "UNKNOWN");
    }
}
