pub mod ast_node;
pub mod visulize;

use lazy_static::lazy_static;
use std::fs::File;
use std::io;
use std::process::Command;
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    sync::Mutex,
};

use crate::error::err_exit;
use crate::lexer::KEYWORD;

use self::ast_node::program::Program;
use self::ast_node::unknown::Unknown;
use self::visulize::Visualizable;

lazy_static! {
    static ref COUNTER: Mutex<Counter> = Mutex::new(Counter::new());
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

    fn write(&self, writer: &mut BufWriter<File>) -> io::Result<()> {
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

    pub(crate) fn put_node(&mut self, id: usize, desc: &str) {
        self.graph
            .push_str(&AstGraph::label(&AstGraph::node_name(id), desc));
    }

    fn node_name(id: usize) -> String {
        format!("node{}", id)
    }

    fn node_link(a: &String, b: &String) -> String {
        format!("\t{} -- {}\n", a, b)
    }

    pub(crate) fn label(node: &str, desc: &str) -> String {
        match KEYWORD.contains_key(desc) {
            true => format!("\t{}[label=\"{}\", color=red]\n", node, desc),
            false => format!("\t{}[label=\"{}\"]\n", node, desc),
        }
    }
}

struct Counter(usize);

impl Counter {
    fn new() -> Self {
        Self(0)
    }

    fn get_id(&mut self) -> usize {
        self.0 += 1;
        self.0
    }
}

pub struct AST {
    program: Program,
    graph: AstGraph,
}

impl AST {
    pub fn new(program: Program) -> AST {
        AST {
            graph: AstGraph::new(),
            program,
        }
    }

    pub fn gen_id() -> usize {
        COUNTER.lock().unwrap().get_id()
    }

    pub fn vis(&mut self, to_path: &str) {
        let mut writer = BufWriter::new(
            OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(to_path)
                .unwrap(),
        );

        self.program.draw(&mut self.graph);
        match self.graph.write(&mut writer) {
            Ok(_) => {}
            Err(err) => err_exit(err),
        }

        Command::new("dot")
            .arg("-Tpng")
            .arg(to_path)
            .arg("-o")
            .arg(to_path.replace("dot", "png"))
            .spawn()
            .expect("dot command failed to start");
    }
}

#[derive(Debug, Default)]
pub struct ASTNode<T: Visualizable> {
    id: usize,
    kind: T,
}

impl<T: Visualizable> ASTNode<T> {
    pub(crate) fn new(kind: T) -> ASTNode<T> {
        let self_id = AST::gen_id();
        ASTNode { id: self_id, kind }
    }

    fn draw(&self, father_id: usize, graph: &mut AstGraph) {
        graph.put_edge(father_id, self.id);
        self.kind.draw(self.id, graph);
    }
}

impl ASTNode<Unknown> {
    pub fn dummy() -> ASTNode<Unknown> {
        ASTNode {
            id: 0,
            kind: Unknown::new(),
        }
    }
}
