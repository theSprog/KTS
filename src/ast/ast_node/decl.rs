use crate::{
    ast::{visulize::Visualizable, ASTNode, AstGraph, NodeInfo, Span},
    lexer::token_kind::{KeyWordKind, TokenKind},
};

use super::{
    class::{Accesser, ClassHeritage, ClassTail, Extends},
    exp::Exp,
    identifier::Identifier,
    parameter::{FormalPara, FormalParas, TypeAnnotation, TypeParas},
    sig::*,
    stat::VarStat,
    type_::*, source_element::SourceElements,
};

#[derive(Visualizable, Default)]
pub struct ClassDecl {
    abstr: Option<KeyWordKind>,
    class_name: ASTNode<Identifier>,
    type_paras: Option<ASTNode<TypeParas>>,
    class_heritage: Option<ASTNode<ClassHeritage>>,
    class_tail: ASTNode<ClassTail>,
}
impl ClassDecl {
    pub(crate) fn set_abstract(&mut self) {
        self.abstr = Some(KeyWordKind::Abstract);
    }

    pub(crate) fn set_class_name(&mut self, class_name: ASTNode<Identifier>) {
        self.class_name = class_name;
    }

    pub(crate) fn set_type_paras(&mut self, type_paras: ASTNode<TypeParas>) {
        self.type_paras = Some(type_paras);
    }

    pub(crate) fn set_class_heritage(&mut self, class_heritage: ASTNode<ClassHeritage>) {
        self.class_heritage = Some(class_heritage);
    }

    pub(crate) fn set_class_tail(&mut self, class_tail: ASTNode<ClassTail>) {
        self.class_tail = class_tail;
    }
}

#[derive(Visualizable, Default)]
pub struct InterfaceDecl {
    export: Option<KeyWordKind>,
    declare: Option<KeyWordKind>,
    interface_name: ASTNode<Identifier>,
    type_paras: Option<ASTNode<TypeParas>>,
    extends: Vec<ASTNode<Extends>>,
    object_type: ASTNode<ObjectType>,
}
impl InterfaceDecl {
    pub(crate) fn set_export(&mut self) {
        self.export = Some(KeyWordKind::Export);
    }

    pub(crate) fn set_declare(&mut self) {
        self.declare = Some(KeyWordKind::Declare);
    }

    pub(crate) fn set_interface_name(&mut self, interface_name: ASTNode<Identifier>) {
        self.interface_name = interface_name;
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

#[derive(Visualizable)]
pub struct AbsDecl {
    abs_member: ASTNode<AbsMember>,
}

impl AbsDecl {
    pub(crate) fn new(abs_member: ASTNode<AbsMember>) -> Self {
        Self { abs_member }
    }
}

#[derive(Visualizable)]
pub enum AbsMember {
    AbsMethod(ASTNode<AbsMethod>),
    AbsVar(ASTNode<AbsVar>),
    AbsAccesser(ASTNode<Accesser>),
}

#[derive(Visualizable, Default)]
pub struct AbsMethod {
    identifier: ASTNode<Identifier>,
    call_sig: ASTNode<CallSig>,
}

impl AbsMethod {
    pub(crate) fn set_identifier(&mut self, identifier: &str) {}

    pub(crate) fn set_call_sig(&mut self, call_sig: ASTNode<CallSig>) {}

    pub(crate) fn new(identifier: ASTNode<Identifier>, call_sig: ASTNode<CallSig>) -> Self {
        Self {
            identifier,
            call_sig,
        }
    }
}

#[derive(Visualizable)]
pub struct AbsVar {
    var_stat: ASTNode<VarStat>,
}

impl AbsVar {
    pub(crate) fn new(var_stat: ASTNode<VarStat>) -> Self {
        Self { var_stat }
    }
}

#[derive(Visualizable, Default)]
pub struct FuncExpDecl {
    func_name: Option<ASTNode<Identifier>>,
    formal_paras: Option<ASTNode<FormalParas>>,
    type_annotation: Option<ASTNode<TypeAnnotation>>,
    func_body: ASTNode<FuncBody>,
}
impl FuncExpDecl {
    pub(crate) fn set_func_name(&mut self, func_name: ASTNode<Identifier>) {
        self.func_name = Some(func_name);
    }

    pub(crate) fn set_formal_paras(&mut self, formal_paras: ASTNode<FormalParas>) {
        self.formal_paras = Some(formal_paras);
    }

    pub(crate) fn set_type_annotation(&mut self, type_annotation: ASTNode<TypeAnnotation>) {
        self.type_annotation = Some(type_annotation);
    }

    pub(crate) fn set_func_body(&mut self, func_body: ASTNode<FuncBody>) {
        self.func_body = func_body;
    }
}

#[derive(Visualizable, Default)]
pub struct FuncBody {
    source_elements: Option<ASTNode<SourceElements>>,
}
impl FuncBody {
    pub(crate) fn set_func_body(&mut self, source_elements: ASTNode<SourceElements>) {
        self.source_elements = Some(source_elements);
    }
}

#[derive(Visualizable, Default)]
pub struct ArrowFuncExpDecl {
    async_: Option<KeyWordKind>,
    formal_paras: ASTNode<FormalParas>,
    type_annotation: Option<ASTNode<TypeAnnotation>>,
    func_body: ASTNode<ArrowFuncBody>,
}
impl ArrowFuncExpDecl {
    pub(crate) fn set_async(&mut self) {
        self.async_ = Some(KeyWordKind::Async);
    }

    // (a, b, c) => ...
    pub(crate) fn set_formal_paras(&mut self, formal_paras: ASTNode<FormalParas>) {
        self.formal_paras = formal_paras;
    }

    pub(crate) fn set_type_annotation(&mut self, type_annotation: ASTNode<TypeAnnotation>) {
        self.type_annotation = Some(type_annotation);
    }

    pub(crate) fn set_func_body(&mut self, func_body: ASTNode<ArrowFuncBody>) {
        self.func_body = func_body;
    }
}

#[derive(Visualizable)]
pub enum ArrowFuncBody {
    FuncBody(ASTNode<FuncBody>),
    ExpBody(ASTNode<Exp>),
}

impl Default for ArrowFuncBody {
    fn default() -> Self {
        ArrowFuncBody::FuncBody(ASTNode::new(FuncBody::default(), Span::default()))
    }
}

#[derive(Visualizable, Default)]
pub struct GenFuncDecl {}

#[derive(Visualizable, Default)]
pub struct NamespaceDecl {
    names: Vec<ASTNode<Identifier>>,
}
impl NamespaceDecl {
    pub(crate) fn push_name(&mut self, name: ASTNode<Identifier>) {
        self.names.push(name);
    }
}
