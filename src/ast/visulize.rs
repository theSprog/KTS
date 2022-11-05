use super::{ASTNode, AST_GRAPH};

pub trait Visualizable {
    fn draw(&self, id: usize);
}

impl<T: Visualizable> Visualizable for Vec<ASTNode<T>> {
    fn draw(&self, id: usize) {
        for node in self {
            node.draw();
            AST_GRAPH::put_edge(id, node.id);
        }
    }
}

impl<T: Visualizable> Visualizable for Vec<Box<ASTNode<T>>> {
    fn draw(&self, id: usize) {
        for node in self {
            node.draw();
            AST_GRAPH::put_edge(id, node.id);
        }
    }
}

impl<T: Visualizable> Visualizable for Option<ASTNode<T>> {
    fn draw(&self, id: usize) {
        match self {
            Some(node) => {
                node.draw();
                AST_GRAPH::put_edge(id, node.id);
            }
            None => (),
        }
    }
}

impl<T: Visualizable> Visualizable for Option<Box<ASTNode<T>>> {
    fn draw(&self, id: usize) {
        match self {
            Some(nodes) => {
                nodes.draw();
                AST_GRAPH::put_edge(id, nodes.id);
            }
            None => (),
        }
    }
}

impl<T: Visualizable> Visualizable for Option<Vec<ASTNode<T>>> {
    fn draw(&self, id: usize) {
        match self {
            Some(nodes) => {
                nodes.draw(id);
            }
            None => (),
        }
    }
}

impl<T: Visualizable> Visualizable for Option<Vec<Box<ASTNode<T>>>> {
    fn draw(&self, id: usize) {
        match self {
            Some(nodes) => {
                nodes.draw(id);
            }
            None => (),
        }
    }
}
