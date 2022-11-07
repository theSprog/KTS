use super::{ASTNode, AstGraph};

pub trait Visualizable {
    fn draw(&self, father_id: usize, graph: &mut AstGraph);
}

impl<T: Visualizable> Visualizable for Vec<ASTNode<T>> {
    fn draw(&self, father_id: usize, graph: &mut AstGraph) {
        // for node in self {
        //     node.draw(father_id, graph);
        // }

        self.into_iter()
            .for_each(|node| node.draw(father_id, graph));
    }
}

impl<T: Visualizable> Visualizable for Vec<Box<ASTNode<T>>> {
    fn draw(&self, father_id: usize, graph: &mut AstGraph) {
        // for node in self {
        //     node.draw(father_id, graph);
        // }

        self.into_iter()
            .for_each(|node| node.draw(father_id, graph));
    }
}

impl<T: Visualizable> Visualizable for Option<ASTNode<T>> {
    fn draw(&self, father_id: usize, graph: &mut AstGraph) {
        // match self {
        //     Some(node) => {
        //         node.draw(father_id, graph);
        //     }
        //     None => (),
        // }

        if let Some(node) = self {
            node.draw(father_id, graph);
        }
    }
}

impl<T: Visualizable> Visualizable for Option<Box<ASTNode<T>>> {
    fn draw(&self, father_id: usize, graph: &mut AstGraph) {
        // match self {
        //     Some(nodes) => {
        //         nodes.draw(father_id, graph);
        //     }
        //     None => (),
        // }

        if let Some(node) = self {
            node.draw(father_id, graph);
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
