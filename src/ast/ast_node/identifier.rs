use crate::ast::{visulize::Visualizable, AstGraph};

#[derive(Default)]
pub struct Identifier {
    context: String,
}

impl Identifier {
    // 注意: Identifier 虽然输入的是引用，但会在内部 clone 一个完整的 string
    // 因此他并不在乎外部传入变量的生命周期
    pub fn new(context: &str) -> Self {
        Identifier {
            context: context.to_string(),
        }
    }
}

impl Visualizable for Identifier {
    fn draw(&self, id: usize, graph: &mut AstGraph) {
        graph.put_node(id, &self.context);
    }
    // fn draw(&self, id: usize) {
    //     AST_GRAPH::put_node(id, &self.context);
    // }
}
