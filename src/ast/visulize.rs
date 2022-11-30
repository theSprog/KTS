use super::{ASTNode, NodeInfo, Span, AST};
use lazy_static::lazy_static;
use std::{
    fs::File,
    io::{self, BufWriter, Write},
    sync::Mutex,
};

pub trait Visualizable {
    fn draw(&self, self_info: NodeInfo, graph: &mut AstGraph);
}

impl<T: Visualizable> Visualizable for Vec<ASTNode<T>> {
    fn draw(&self, self_info: NodeInfo, graph: &mut AstGraph) {
        self.iter().for_each(|node| node.draw(self_info, graph));
    }
}

impl<T: Visualizable> Visualizable for Vec<T> {
    fn draw(&self, self_info: NodeInfo, graph: &mut AstGraph) {
        self.iter().for_each(|node| node.draw(self_info, graph));
    }
}

impl<T: Visualizable> Visualizable for Vec<Box<ASTNode<T>>> {
    fn draw(&self, self_info: NodeInfo, graph: &mut AstGraph) {
        self.iter().for_each(|node| node.draw(self_info, graph));
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

pub struct AstGraph {
    graph: String,
}

impl AstGraph {
    pub fn new() -> AstGraph {
        Self {
            graph: String::new(),
        }
    }

    pub(crate) fn write(&self, writer: &mut BufWriter<File>) -> io::Result<()> {
        writer.write_all(b"graph vis {\n")?;
        writer.write_all(self.graph.as_bytes())?;
        writer.write_all(b"}\n")?;
        writer.flush()?;
        Ok(())
    }

    pub(crate) fn put_edge(&mut self, father: usize, child: usize) {
        self.graph.push_str(&AstGraph::node_link(
            &AstGraph::node_name(father),
            &AstGraph::node_name(child),
        ));
    }

    pub(crate) fn put_node(&mut self, info: NodeInfo, desc: &str) {
        self.graph.push_str(&AstGraph::label(info, desc));
    }

    fn node_name(id: usize) -> String {
        format!("node{}", id)
    }

    fn node_link(a: &String, b: &String) -> String {
        format!("\t{} -- {}\n", a, b)
    }

    pub(crate) fn label(info: NodeInfo, desc: &str) -> String {
        let node = &AstGraph::node_name(info.id);

        if info.span.begin == 0 {
            assert_eq!(info.span.begin, info.span.end);
            format!("\t{}[label=\"{}\", color=red]\n", node, desc)
        } else {
            format!(
                "\t{}[label=\"{}\"]\n",
                node,
                format!("{}\n[{}, {}]", desc, info.span.begin, info.span.end)
            )
        }
    }
}

lazy_static! {
    pub static ref COUNTER: Mutex<Counter> = Mutex::new(Counter::new());
}

pub struct Counter(usize);

impl Counter {
    fn new() -> Self {
        Self(0)
    }

    pub(super) fn get_id(&mut self) -> usize {
        self.0 += 1;
        self.0
    }
}
