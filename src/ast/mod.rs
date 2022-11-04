pub mod visulize;

use lazy_static::lazy_static;
use std::fmt::Display;
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
use self::visulize::Visualizable;

lazy_static! {
    static ref COUNTER: Mutex<Counter> = Mutex::new(Counter::new());
    pub static ref AST_GRAPH: Mutex<String> = Mutex::new(String::new());
}

impl Display for AST_GRAPH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AST_GRAPH: {}", self.lock().unwrap())
    }
}

impl AST_GRAPH {
    pub fn put_node(id: usize, desc: &str) {
        AST_GRAPH
            .lock()
            .unwrap()
            .push_str(&AST::label(&AST::node_name(id), desc));
    }

    fn put_edge(id1: usize, id2: usize) {
        AST_GRAPH
            .lock()
            .unwrap()
            .push_str(&AST::node_link(&AST::node_name(id1), &AST::node_name(id2)));
    }

    fn draw(ast: &AST, to_path: &str) -> io::Result<()> {
        let vis_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(to_path)
            .unwrap();

        let mut writer = BufWriter::new(vis_file);

        // 由于 AST_GRAPH 是全局变量，所以使用前必须要先清除
        AST_GRAPH.lock().unwrap().clear();
        ast.program.draw();

        writer.write_all(b"graph vis {\n")?;
        writer.write_all(AST_GRAPH.lock().unwrap().as_bytes())?;
        writer.write_all(b"}\n")?;
        writer.flush()?;

        Command::new("dot")
            .arg("-Tpng")
            .arg(to_path)
            .arg("-o")
            .arg(to_path.replace("dot", "png"))
            .spawn()
            .expect("dot command failed to start");

        Ok(())
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
    program: ASTNode<Program>,
}

impl AST {
    pub fn new(program: ASTNode<Program>) -> AST {
        AST { program }
    }

    pub fn vis(&self, to_path: &str) {
        match AST_GRAPH::draw(&self, to_path) {
            Ok(_) => {}
            Err(err) => err_exit(err),
        };
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

#[derive(Debug, Default)]
pub struct ASTNode<T: Visualizable> {
    id: usize,
    kind: T,
}

impl<T: Visualizable> ASTNode<T> {
    pub(crate) fn new(kind: T) -> ASTNode<T> {
        let self_id = COUNTER.lock().unwrap().get_id();
        ASTNode { id: self_id, kind }
    }

    fn draw(&self) {
        self.kind.draw(self.id)
    }
}

pub mod ast_node;
