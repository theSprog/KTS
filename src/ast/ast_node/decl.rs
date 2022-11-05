use crate::{
    ast::{visulize::Visualizable, ASTNode, AST_GRAPH},
    lexer::token_kind::TokenKind,
};

use super::{body::FuncBody, call_sig::CallSig, identifier::Identifier, literal::Value};

pub struct ClassDecl {}
impl Visualizable for ClassDecl {
    fn draw(&self, id: usize) {
        todo!()
    }
}

pub struct AbsDecl {}
impl Visualizable for AbsDecl {
    fn draw(&self, id: usize) {
        todo!()
    }
}
pub struct FuncDecl {
    func_name: ASTNode<Identifier>,
    call_sig: ASTNode<CallSig>,
    func_body: Option<ASTNode<FuncBody>>,
}
impl FuncDecl {
    pub(crate) fn new() -> Self {
        Self {
            func_name: Default::default(),
            call_sig: Default::default(),
            func_body: None,
        }
    }

    pub(crate) fn set_func_name(&mut self, func_name: &str) {
        self.func_name = ASTNode::new(Identifier::new(func_name));
    }

    pub(crate) fn set_call_sig(&mut self, call_sig: ASTNode<CallSig>) {
        self.call_sig = call_sig;
    }

    pub(crate) fn set_func_body(&mut self, func_body: ASTNode<FuncBody>) {
        self.func_body = Some(func_body);
    }
}

impl Visualizable for FuncDecl {
    fn draw(&self, id: usize) {
        AST_GRAPH::put_node(id, "FuncDecl");

        AST_GRAPH::put_edge(id, self.func_name.id);
        self.func_name.draw();

        AST_GRAPH::put_edge(id, self.call_sig.id);
        self.call_sig.draw();

        if let Some(func_body) = &self.func_body {
            AST_GRAPH::put_edge(id, func_body.id);
            func_body.draw()
        }
    }
}

pub struct FuncExpDecl {}
impl Visualizable for FuncExpDecl {
    fn draw(&self, id: usize) {
        todo!()
    }
}

pub struct GenFuncDecl {}
impl Visualizable for GenFuncDecl {
    fn draw(&self, id: usize) {
        todo!()
    }
}
