use super::{ASTNode, AstGraph, NodeInfo, Span, AST};

pub trait Visualizable {
    fn draw(&self, self_info: NodeInfo, graph: &mut AstGraph);
}

impl<T: Visualizable> Visualizable for Vec<ASTNode<T>> {
    fn draw(&self, self_info: NodeInfo, graph: &mut AstGraph) {
        self.into_iter()
            .for_each(|node| node.draw(self_info, graph));
    }
}

impl<T: Visualizable> Visualizable for Vec<T> {
    fn draw(&self, self_info: NodeInfo, graph: &mut AstGraph) {
        self.into_iter()
            .for_each(|node| node.draw(self_info, graph));
    }
}

impl<T: Visualizable> Visualizable for Vec<Box<ASTNode<T>>> {
    fn draw(&self, self_info: NodeInfo, graph: &mut AstGraph) {
        self.into_iter()
            .for_each(|node| node.draw(self_info, graph));
    }
}

impl<T: Visualizable> Visualizable for Option<ASTNode<T>> {
    fn draw(&self, self_info: NodeInfo, graph: &mut AstGraph) {
        if let Some(node) = self {
            node.draw(self_info, graph);
        }
    }
}

impl<T: Visualizable> Visualizable for Option<T> {
    fn draw(&self, father_info: NodeInfo, graph: &mut AstGraph) {
        if let Some(node) = self {
            let self_id = AST::gen_id();
            graph.put_edge(father_info.id, self_id);
            node.draw(NodeInfo::new(self_id, Span::default()), graph);
        }
    }
}

impl<T: Visualizable> Visualizable for Option<Box<ASTNode<T>>> {
    fn draw(&self, self_info: NodeInfo, graph: &mut AstGraph) {
        if let Some(node) = self {
            node.draw(self_info, graph);
        }
    }
}

// Option Vec 真的存在么
// Vec 为空不就已经代表 Option::None 了么
// impl<T: Visualizable> Visualizable for Option<Vec<ASTNode<T>>> {
//     fn draw(&self, id: usize, graph: &mut AstGraph) {
//         match self {
//             Some(nodes) => {
//                 nodes.draw(id, graph);
//             }
//             None => (),
//         }
//     }
// }

// impl<T: Visualizable> Visualizable for Option<Vec<Box<ASTNode<T>>>> {
//     fn draw(&self, id: usize, graph: &mut AstGraph) {
//         match self {
//             Some(nodes) => {
//                 nodes.draw(id, graph);
//             }
//             None => (),
//         }
//     }
// }
