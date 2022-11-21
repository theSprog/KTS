pub mod ast_node;
pub mod visulize;

use lazy_static::lazy_static;
use std::fs::{self, File};
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
            format!("\t{}[label=\"{}\", color=red]\n", node, format!("{}", desc))
        } else {
            format!(
                "\t{}[label=\"{}\"]\n",
                node,
                format!("{}\n[{}, {}]", desc, info.span.begin, info.span.end)
            )
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
    program: ASTNode<Program>,
    graph: AstGraph,
}

impl AST {
    pub fn new(program: ASTNode<Program>) -> AST {
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

        self.program.draw(NodeInfo::default(), &mut self.graph);
        match self.graph.write(&mut writer) {
            Ok(_) => {}
            Err(err) => err_exit(err),
        }

        let png_path = &to_path.replace("dot", "png");
        let dot_path = to_path;
        Command::new("dot")
            .args(["-Tpng", dot_path, "-o", png_path])
            .spawn()
            .expect("dot command failed to start")
            .wait()
            .expect("dot command failed to run");

        fs::remove_file(dot_path).unwrap();
    }
}

#[derive(Debug, Default)]
pub struct ASTNode<T: Visualizable> {
    pub(crate) info: NodeInfo,
    pub(crate) context: Box<T>,
}

impl<T: Visualizable> ASTNode<T> {
    pub(crate) fn new(context: T, span: Span) -> ASTNode<T> {
        let info = NodeInfo::new(AST::gen_id(), span);
        ASTNode {
            info,
            context: Box::new(context),
        }
    }

    fn draw(&self, father_info: NodeInfo, graph: &mut AstGraph) {
        if father_info.id != 0 {
            graph.put_edge(father_info.id, self.info.id);
        }

        // if it is true, there must be something wrong
        assert_ne!(self.info.id, 0);

        self.context.draw(self.info, graph);
    }

    pub(crate) fn ctx(self) -> T {
        *self.context
    }

    pub(crate) fn ctx_ref(&self) -> &T {
        &self.context
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Span {
    pub(crate) begin: usize,
    pub(crate) end: usize,
}

impl Span {
    pub(crate) fn new(begin: usize, end: usize) -> Self {
        Self { begin, end }
    }

    pub(crate) fn get_begin(&self) -> usize {
        self.begin
    }

    pub(crate) fn get_end(&self) -> usize {
        self.end
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct NodeInfo {
    pub(crate) id: usize,
    pub(crate) span: Span,
}

impl NodeInfo {
    fn new(id: usize, span: Span) -> NodeInfo {
        Self { id, span }
    }
}
