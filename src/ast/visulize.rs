use super::{ASTNode, AstGraph};

pub trait Visualizable {
    fn draw(&self, id: usize, graph: &mut AstGraph);
}

impl<T: Visualizable> Visualizable for Vec<ASTNode<T>> {
    fn draw(&self, id: usize, graph: &mut AstGraph) {
        for node in self {
            node.draw(graph);
            graph.put_edge(id, node.id);
        }
    }
}

impl<T: Visualizable> Visualizable for Vec<Box<ASTNode<T>>> {
    fn draw(&self, id: usize, graph: &mut AstGraph) {
        for node in self {
            node.draw(graph);
            graph.put_edge(id, node.id);
        }
    }
}

impl<T: Visualizable> Visualizable for Option<ASTNode<T>> {
    fn draw(&self, id: usize, graph: &mut AstGraph) {
        match self {
            Some(node) => {
                node.draw(graph);
                graph.put_edge(id, node.id);
            }
            None => (),
        }
    }
}

impl<T: Visualizable> Visualizable for Option<Box<ASTNode<T>>> {
    fn draw(&self, id: usize, graph: &mut AstGraph) {
        match self {
            Some(nodes) => {
                nodes.draw(graph);
                graph.put_edge(id, nodes.id);
            }
            None => (),
        }
    }
}

// Option Vec 真的存在么
// Vec 为空不就已经代表 Option::None 了么
impl<T: Visualizable> Visualizable for Option<Vec<ASTNode<T>>> {
    fn draw(&self, id: usize, graph: &mut AstGraph) {
        match self {
            Some(nodes) => {
                nodes.draw(id, graph);
            }
            None => (),
        }
    }
}

impl<T: Visualizable> Visualizable for Option<Vec<Box<ASTNode<T>>>> {
    fn draw(&self, id: usize, graph: &mut AstGraph) {
        match self {
            Some(nodes) => {
                nodes.draw(id, graph);
            }
            None => (),
        }
    }
}
