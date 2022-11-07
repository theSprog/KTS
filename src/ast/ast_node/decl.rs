use crate::{
    ast::{visulize::Visualizable, ASTNode, AstGraph},
    lexer::token_kind::{KeyWordKind},
};

use super::{
    body::FuncBody,
    call_sig::CallSig,
    class::{ClassHeritage, ClassTail},
    identifier::Identifier,
    parameter::TypeParas,
};

#[derive(Visualizable, Default)]
pub struct ClassDecl {
    abstr: Option<ASTNode<KeyWordKind>>,
    class_name: ASTNode<Identifier>,
    type_paras: Option<ASTNode<TypeParas>>,
    class_heritage: ASTNode<ClassHeritage>,
    class_tail: ASTNode<ClassTail>,
}
impl ClassDecl {
    pub(crate) fn set_abstract(&mut self) {
        self.abstr = Some(ASTNode::new(KeyWordKind::Abstract));
    }

    pub(crate) fn set_class_name(&mut self, class_name: &str) {
        self.class_name = ASTNode::new(Identifier::new(class_name));
    }

    pub(crate) fn set_type_paras(&mut self, type_paras: ASTNode<TypeParas>) {
        self.type_paras = Some(type_paras);
    }

    pub(crate) fn set_class_heritage(&mut self, class_heritage: ASTNode<ClassHeritage>) {
        self.class_heritage = class_heritage;
    }

    pub(crate) fn set_class_tail(&mut self, class_tail: ASTNode<ClassTail>) {
        self.class_tail = class_tail;
    }
}

pub struct AbsDecl {}
impl Visualizable for AbsDecl {
    fn draw(&self, id: usize, graph: &mut AstGraph) {
        todo!()
    }
}

#[derive(Visualizable, Default)]
pub struct FuncDecl {
    func_name: ASTNode<Identifier>,
    call_sig: ASTNode<CallSig>,
    func_body: Option<ASTNode<FuncBody>>,
}
impl FuncDecl {
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

pub struct FuncExpDecl {}
impl Visualizable for FuncExpDecl {
    fn draw(&self, id: usize, graph: &mut AstGraph) {
        todo!()
    }
}

pub struct GenFuncDecl {}
impl Visualizable for GenFuncDecl {
    fn draw(&self, id: usize, graph: &mut AstGraph) {
        todo!()
    }
}
