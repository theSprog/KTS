use crate::ast::visulize::Visualizable;

pub struct EOS {}
impl Visualizable for EOS {
    fn draw(&self, id: usize, graph: &mut crate::ast::AstGraph) {
        todo!()
    }
}
