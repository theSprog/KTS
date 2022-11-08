use crate::ast::ASTNode;
use crate::ast::AstGraph;
use crate::ast::Visualizable;
use crate::lexer::token_kind::KeyWordKind;
use crate::lexer::token_kind::TokenKind;

use super::body::FuncBody;
use super::decl::AbsDecl;
use super::exp::Exp;
use super::identifier::Identifier;
use super::parameter::FormalParas;
use super::parameter::TypeAnnotation;
use super::sig::IndexSig;
use super::sig::*;
use super::type_::*;

pub enum AccessModifier {
    Public,
    Protected,
    Private,
}

impl Visualizable for AccessModifier {
    fn draw(&self, self_id: usize, graph: &mut AstGraph) {
        match self {
            AccessModifier::Public => {
                graph.put_node(self_id, "public");
            }
            AccessModifier::Protected => {
                graph.put_node(self_id, "protected");
            }
            AccessModifier::Private => {
                graph.put_node(self_id, "private");
            }
        }
    }
}

#[derive(Visualizable, Default)]
pub struct ClassHeritage {
    extends: Option<ASTNode<Extends>>,
    implemented: Option<ASTNode<Implement>>,
}
impl ClassHeritage {
    pub(crate) fn set_extends(&mut self, extend: Extends) {
        self.extends = Some(ASTNode::new(extend));
    }

    pub(crate) fn set_implement(&mut self, implemented: Implement) {
        self.implemented = Some(ASTNode::new(implemented));
    }
}

#[derive(Visualizable, Default)]
pub struct Extends {
    type_ref: ASTNode<TypeRef>,
}
impl Extends {
    pub(crate) fn new(extend: ASTNode<TypeRef>) -> Self {
        Self { type_ref: extend }
    }
}

#[derive(Visualizable, Default)]
pub struct Implement {
    type_refs: Vec<ASTNode<TypeRef>>,
}
impl Implement {
    pub(crate) fn push_implemented(&mut self, implemented: ASTNode<TypeRef>) {
        self.type_refs.push(implemented);
    }
}

#[derive(Visualizable, Default)]
pub struct ClassTail {
    class_elements: Vec<ASTNode<ClassElement>>,
}

impl ClassTail {
    pub(crate) fn push_class_element(&mut self, class_element: ASTNode<ClassElement>) {
        self.class_elements.push(class_element)
    }
}

#[derive(Visualizable)]
pub enum ClassElement {
    ConstructorDecl(ASTNode<ConstructorDecl>),
    PropertyMemberDecl(ASTNode<PropertyMemberDecl>),
    IndexMemberDecl(ASTNode<IndexMemberDecl>),
}

#[derive(Visualizable, Default)]
pub struct ConstructorDecl {
    access_modifier: Option<ASTNode<AccessModifier>>,
    formal_paras: ASTNode<FormalParas>,
    func_body: ASTNode<FuncBody>,
}

impl ConstructorDecl {
    pub(crate) fn set_access_modifier(&mut self, access_modifier: ASTNode<AccessModifier>) {
        self.access_modifier = Some(access_modifier);
    }

    pub(crate) fn set_formal_paras(&mut self, formal_paras: ASTNode<FormalParas>) {
        self.formal_paras = formal_paras;
    }

    pub(crate) fn set_func_body(&mut self, func_body: ASTNode<FuncBody>) {
        self.func_body = func_body;
    }
}

#[derive(Visualizable)]
pub enum PropertyMemberDecl {
    PropertyDeclExp(ASTNode<PropertyDeclExp>),
    MethodDeclExp(ASTNode<MethodDeclExp>),
    GetterSetterDeclExp(ASTNode<GetterSetterDeclExp>),
    AbsMemberDecl(ASTNode<AbsDecl>),
}

#[derive(Visualizable, Default)]
pub struct PropertyDeclExp {
    access_modifier: Option<ASTNode<AccessModifier>>,
    static_: Option<ASTNode<KeyWordKind>>,
    readonly: Option<ASTNode<KeyWordKind>>,
    identifier: ASTNode<Identifier>,
    question_mark: Option<ASTNode<TokenKind>>,
    type_annotation: Option<ASTNode<TypeAnnotation>>,
    initializer: Option<ASTNode<Exp>>,
}
impl PropertyDeclExp {
    pub(crate) fn set_access_modifier(&mut self, access_modifier: ASTNode<AccessModifier>) {
        self.access_modifier = Some(access_modifier);
    }

    pub(crate) fn set_static(&mut self) {
        self.static_ = Some(ASTNode::new(KeyWordKind::Static));
    }

    pub(crate) fn set_readonly(&mut self) {
        self.readonly = Some(ASTNode::new(KeyWordKind::ReadOnly));
    }

    pub(crate) fn set_identifier(&mut self, identifier: &str) {
        self.identifier = ASTNode::new(Identifier::new(identifier));
    }

    pub(crate) fn set_question_mark(&mut self) {
        self.question_mark = Some(ASTNode::new(TokenKind::QuestionMark));
    }

    pub(crate) fn set_type_annotation(&mut self, type_annotation: ASTNode<TypeAnnotation>) {
        self.type_annotation = Some(type_annotation);
    }

    pub(crate) fn set_initializer(&mut self, single_exp: ASTNode<Exp>) {
        self.initializer = Some(single_exp);
    }
}

#[derive(Visualizable, Default)]
pub struct MethodDeclExp {
    access_modifier: Option<ASTNode<AccessModifier>>,
    static_: Option<ASTNode<KeyWordKind>>,
    async_: Option<ASTNode<KeyWordKind>>,
    identifier: ASTNode<Identifier>,
    call_signature: ASTNode<CallSig>,
    func_body: Option<ASTNode<FuncBody>>,
}

impl MethodDeclExp {
    pub(crate) fn set_access_modifier(&mut self, access_modifier: ASTNode<AccessModifier>) {
        self.access_modifier = Some(access_modifier);
    }

    pub(crate) fn set_static(&mut self) {
        self.static_ = Some(ASTNode::new(KeyWordKind::Static));
    }

    pub(crate) fn set_async(&mut self) {
        self.async_ = Some(ASTNode::new(KeyWordKind::Async));
    }

    pub(crate) fn set_identifier(&mut self, identifier: &str) {
        self.identifier = ASTNode::new(Identifier::new(identifier));
    }

    pub(crate) fn set_call_sig(&mut self, call_signature: ASTNode<CallSig>) {
        self.call_signature = call_signature;
    }

    pub(crate) fn set_func_body(&mut self, func_body: ASTNode<FuncBody>) {
        self.func_body = Some(func_body);
    }
}

#[derive(Visualizable)]
pub struct GetterSetterDeclExp {
    access_modifier: Option<ASTNode<AccessModifier>>,
    static_: Option<ASTNode<KeyWordKind>>,
    accesser: ASTNode<Accesser>,
}
impl GetterSetterDeclExp {
    pub(crate) fn new(
        access_modifier: Option<ASTNode<AccessModifier>>,
        static_: bool,
        accesser: ASTNode<Accesser>,
    ) -> Self {
        Self {
            access_modifier,
            static_: match static_ {
                true => Some(ASTNode::new(KeyWordKind::Static)),
                false => None,
            },
            accesser,
        }
    }
}

#[derive(Visualizable)]
pub enum Accesser {
    SetAccessor(SetAccesser),
    GetAccessor(GetAccesser),
}

#[derive(Visualizable, Default)]
pub struct SetAccesser {
    identifier: ASTNode<Identifier>,
    parameter: ASTNode<Identifier>,
    type_annotation: Option<ASTNode<TypeAnnotation>>,
    func_body: ASTNode<FuncBody>,
}
impl SetAccesser {
    pub(crate) fn set_identifier(&mut self, identifier: &str) {
        self.identifier = ASTNode::new(Identifier::new(identifier));
    }

    pub(crate) fn set_parameter(&mut self, identifier: &str) {
        self.parameter = ASTNode::new(Identifier::new(identifier));
    }

    pub(crate) fn set_type_annotation(&mut self, type_annotation: ASTNode<TypeAnnotation>) {
        self.type_annotation = Some(type_annotation);
    }

    pub(crate) fn set_func_body(&mut self, func_body: ASTNode<FuncBody>) {
        self.func_body = func_body;
    }
}

#[derive(Visualizable, Default)]
pub struct GetAccesser {
    identifier: ASTNode<Identifier>,
    type_annotation: Option<ASTNode<TypeAnnotation>>,
    func_body: ASTNode<FuncBody>,
}
impl GetAccesser {
    pub(crate) fn set_identifier(&mut self, identifier: &str) {
        self.identifier = ASTNode::new(Identifier::new(identifier));
    }

    pub(crate) fn set_type_annotation(&mut self, type_annotation: ASTNode<TypeAnnotation>) {
        self.type_annotation = Some(type_annotation);
    }

    pub(crate) fn set_func_body(&mut self, func_body: ASTNode<FuncBody>) {
        self.func_body = func_body;
    }
}

#[derive(Visualizable, Default)]
pub struct IndexMemberDecl {
    index_sig: ASTNode<IndexSig>,
}
impl IndexMemberDecl {
    pub(crate) fn set_index_sig(&mut self, index_sig: ASTNode<IndexSig>) {
        self.index_sig = index_sig;
    }
}
