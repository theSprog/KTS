use crate::{
    ast::{visulize::Visualizable, ASTNode, AstGraph},
    lexer::token_kind::{KeyWordKind, TokenKind},
};

use super::{
    body::FuncBody,
    class::{ClassHeritage, ClassTail, Extends, Accesser},
    exp::Exp,
    identifier::Identifier,
    parameter::{FormalPara, FormalParas, TypeAnnotation, TypeParas},
    sig::*,
    stat::VarStat,
    type_::*,
};

#[derive(Visualizable, Default)]
pub struct ClassDecl {
    abstr: Option<ASTNode<KeyWordKind>>,
    class_name: ASTNode<Identifier>,
    type_paras: Option<ASTNode<TypeParas>>,
    class_heritage: Option<ASTNode<ClassHeritage>>,
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
        self.class_heritage = Some(class_heritage);
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

#[derive(Visualizable)]
pub struct AbsDecl {
    abs_member: ASTNode<AbsMember>,
}

impl AbsDecl {
    pub(crate) fn new(abs_member: AbsMember) -> Self {
        Self {
            abs_member: ASTNode::new(abs_member),
        }
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

    pub(crate) fn new(identifier: &str, call_sig: ASTNode<CallSig>) -> Self {
        Self {
            identifier: ASTNode::new(Identifier::new(identifier)),
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
pub struct FuncExpDecl {
    func_name: Option<ASTNode<Identifier>>,
    formal_paras: Option<ASTNode<FormalParas>>,
    type_annotation: Option<ASTNode<TypeAnnotation>>,
    func_body: ASTNode<FuncBody>,
}
impl FuncExpDecl {
    pub(crate) fn set_func_name(&mut self, func_name: &str) {
        self.func_name = Some(ASTNode::new(Identifier::new(func_name)));
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

#[derive(Default)]
pub struct ArrowFuncExpDecl {
    async_: Option<ASTNode<KeyWordKind>>,
    formal_paras: Option<ASTNode<FormalParas>>,
    type_annotation: Option<ASTNode<TypeAnnotation>>,
    func_body: ASTNode<ArrowFuncBody>,
}
impl ArrowFuncExpDecl {
    pub(crate) fn set_async(&mut self) {
        self.async_ = Some(ASTNode::new(KeyWordKind::Async));
    }

    pub(crate) fn set_formal_paras(&mut self, formal_paras: ASTNode<FormalParas>) {
        assert!(self.formal_paras.is_none());

        self.formal_paras = Some(formal_paras);
    }

    pub(crate) fn set_formal_para(&mut self, formal_para: ASTNode<FormalPara>) {
        assert!(self.formal_paras.is_none());

        let mut formal_paras = FormalParas::default();
        formal_paras.push_formal_para(formal_para);
        self.formal_paras = Some(ASTNode::new(formal_paras));
    }

    pub(crate) fn set_type_annotation(&mut self, type_annotation: ASTNode<TypeAnnotation>) {
        self.type_annotation = Some(type_annotation);
    }

    pub(crate) fn set_func_body(&mut self, func_body: ASTNode<FuncBody>) {
        self.func_body = ASTNode::new(ArrowFuncBody::FuncBody(func_body));
    }

    pub(crate) fn set_exp_body(&mut self, exp: ASTNode<Exp>) {
        self.func_body = ASTNode::new(ArrowFuncBody::ExpBody(exp));
    }
}

impl Visualizable for ArrowFuncExpDecl {
    fn draw(&self, self_id: usize, graph: &mut AstGraph) {
        graph.put_node(self_id, "ArrowFuncExpDecl");

        self.formal_paras.draw(self_id, graph);
        self.type_annotation.draw(self_id, graph);
        self.func_body.draw(self_id, graph);
        // match &*self.func_body.context {
        //     ArrowFuncBody::FuncBody(func_body) => {
        //         func_body.draw(self_id, graph);
        //     }
        //     ArrowFuncBody::ExpBody(exp) => {
        //         exp.draw(self_id, graph);
        //     }
        // }
    }
}

#[derive(Visualizable)]
enum ArrowFuncBody {
    FuncBody(ASTNode<FuncBody>),
    ExpBody(ASTNode<Exp>),
}

impl Default for ArrowFuncBody {
    fn default() -> Self {
        ArrowFuncBody::FuncBody(ASTNode::new(FuncBody::default()))
    }
}

#[derive(Visualizable, Default)]
pub struct GenFuncDecl {}

#[derive(Visualizable, Default)]
pub struct NamespaceDecl {}
