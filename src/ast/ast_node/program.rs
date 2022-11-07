use ::visulize::Visualizable;

use crate::ast::{visulize::*, *};

use super::source_elements::SourceElements;

pub struct Program {
    source_elements: Option<ASTNode<SourceElements>>,
}
impl Program {
    pub(crate) fn new() -> Program {
        Self {
            source_elements: None,
        }
    }

    pub(crate) fn set_source_elements(&mut self, source_elements: ASTNode<SourceElements>) {
        self.source_elements = Some(source_elements);
    }

    pub(crate) fn draw(&mut self, graph: &mut AstGraph) {
        let program_id = COUNTER.lock().unwrap().get_id();
        graph.put_node(program_id, "Program");
        self.source_elements.draw(program_id, graph);
    }
}
