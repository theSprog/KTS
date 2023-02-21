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
use self::visulize::{AstGraph, Visualizable, COUNTER};

pub struct AST {
    pub program: ASTNode<Program>,
    pub filename: String,
}

impl AST {
    pub fn new(program: ASTNode<Program>, filename: String) -> AST {
        AST { program, filename }
    }

    pub fn gen_id() -> usize {
        COUNTER.lock().unwrap().get_id()
    }

    pub fn get_program_ref(&self) -> &ASTNode<Program> {
        &self.program
    }

    pub fn vis(&self, to_path: &str) {
        let mut writer = BufWriter::new(
            OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(to_path)
                .unwrap(),
        );

        let mut graph = AstGraph::new();
        self.program.draw(NodeInfo::default(), &mut graph);
        if let Err(err) = graph.write(&mut writer) {
            err_exit(err);
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
        ASTNode {
            info: NodeInfo::new(AST::gen_id(), span),
            context: Box::new(context),
        }
    }

    fn draw(&self, father_info: NodeInfo, graph: &mut AstGraph) {
        if father_info.id != EMPTY_ID {
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

const EMPTY_ID: usize = 0;

#[derive(Debug, Default, Clone, Copy)]
pub struct NodeInfo {
    pub(crate) id: usize,
    pub(crate) span: Span,
}

impl NodeInfo {
    fn new(id: usize, span: Span) -> NodeInfo {
        assert_ne!(id, EMPTY_ID);
        Self { id, span }
    }
}
