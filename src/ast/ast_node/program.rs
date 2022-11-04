// use ::visulize::Vis;

use crate::ast::{visulize::*, *};

use super::source_elements::SourceElements;

#[derive(Vis)]
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
}

// impl Visualizable for Program {
//     fn draw(&self, id: usize) {
//         println!("{}", AST_GRAPH);
//         AST_GRAPH::put_node(id, "Program");
//         self.source_elements.draw(id);
//     }
// }
