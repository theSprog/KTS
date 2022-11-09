use crate::{
    ast::{visulize::Visualizable, ASTNode, AstGraph},
    lexer::token_kind::KeyWordKind,
};

use super::{
    body::FuncBody,
    class::{ClassHeritage, ClassTail, Extends},
    identifier::Identifier,
    parameter::TypeParas,
    sig::*,
    type_::*,
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

#[derive(Visualizable, Default)]
pub struct InterfaceDecl {
    export: Option<ASTNode<KeyWordKind>>,
    declare: Option<ASTNode<KeyWordKind>>,
    interface_name: ASTNode<Identifier>,
    type_paras: Option<ASTNode<TypeParas>>,
    extends: Vec<ASTNode<Extends>>,
    object_type: ASTNode<ObjectType>,
}
impl InterfaceDecl {
    pub(crate) fn set_export(&mut self) {
        self.export = Some(ASTNode::new(KeyWordKind::Export));
    }

    pub(crate) fn set_declare(&mut self) {
        self.declare = Some(ASTNode::new(KeyWordKind::Declare));
    }

    pub(crate) fn set_identifier(&mut self, identifier: &str) {
        self.interface_name = ASTNode::new(Identifier::new(identifier));
    }

    pub(crate) fn push_extends(&mut self, extends: ASTNode<Extends>) {
        self.extends.push(extends);
    }

    pub(crate) fn set_object_type(&mut self, object_type: ASTNode<ObjectType>) {
        self.object_type = object_type;
    }

    pub(crate) fn set_type_paras(&mut self, type_paras: ASTNode<TypeParas>) {
        self.type_paras = Some(type_paras);
    }
}

#[derive(Visualizable, Default)]
pub struct ObjectType {
    type_members: Vec<ASTNode<TypeMember>>,
}
impl ObjectType {
    pub(crate) fn push_type_member(&mut self, type_member: ASTNode<TypeMember>) {
        self.type_members.push(type_member);
    }
}

#[derive(Visualizable)]
pub enum TypeMember {
    PropertySig(ASTNode<PropertySig>),
    MethodSig(ASTNode<MethodSig>),
    CallSig(ASTNode<CallSig>),
    ConstructSig(ASTNode<ConstructSig>),
    IndexSig(ASTNode<IndexSig>),
}

#[derive(Visualizable, Default)]
pub struct AbsDecl {
    identifier: ASTNode<Identifier>,
    call_sig: ASTNode<CallSig>,
}
impl AbsDecl {
    pub(crate) fn set_identifier(&mut self, identifier: &str) {
        self.identifier = ASTNode::new(Identifier::new(identifier));
    }

    pub(crate) fn set_call_sig(&mut self, call_sig: ASTNode<CallSig>) {
        self.call_sig = call_sig;
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

#[derive(Visualizable, Default)]
pub struct FuncExpDecl {}

#[derive(Visualizable, Default)]
pub struct GenFuncDecl {}

#[derive(Visualizable, Default)]
pub struct NamespaceDecl {}
