use crate::ast::{visulize::*, *};

use super::source_element::SourceElements;

#[derive(Default)]
pub struct Program {
    pub(crate) source_elements: Option<ASTNode<SourceElements>>,
}
impl Program {
    pub(crate) fn set_source_elements(&mut self, source_elements: ASTNode<SourceElements>) {
        self.source_elements = Some(source_elements);
    }
}

impl Visualizable for Program {
    fn draw(&self, self_info: NodeInfo, graph: &mut AstGraph) {
        graph.put_node(self_info, "Program");
        self.source_elements.draw(self_info, graph);
    }
}
