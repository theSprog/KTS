use crate::ast::ASTNode;
use crate::ast::AstGraph;
use crate::ast::Visualizable;
use crate::lexer::token_kind::KeyWordKind;

use super::body::FuncBody;
use super::parameter::FormalParas;
use super::type_ref::*;

#[derive(Visualizable, Default)]
pub struct ClassHeritage {
    extends: Option<ASTNode<Extend>>,
    implemented: Option<ASTNode<Implement>>,
}
impl ClassHeritage {
    pub(crate) fn set_extends(&mut self, extend: Extend) {
        self.extends = Some(ASTNode::new(extend));
    }

    pub(crate) fn set_implement(&mut self, implemented: Implement) {
        self.implemented = Some(ASTNode::new(implemented));
    }
}

#[derive(Visualizable, Default)]
pub struct Extend {
    type_ref: ASTNode<TypeRef>,
}
impl Extend {
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

#[derive(Visualizable)]
pub struct ConstructorDecl {
    access_modifier: ASTNode<KeyWordKind>,
    formal_paras: ASTNode<FormalParas>,
    func_body: ASTNode<FuncBody>,
}

impl Default for ConstructorDecl {
    fn default() -> Self {
        Self {
            access_modifier: ASTNode::new(KeyWordKind::Public),
            formal_paras: Default::default(),
            func_body: Default::default(),
        }
    }
}

impl ConstructorDecl {
    pub(crate) fn set_access(&mut self, access_modifier: KeyWordKind) {
        self.access_modifier = ASTNode::new(access_modifier);
    }

    pub(crate) fn set_formal_paras(&mut self, formal_paras: ASTNode<FormalParas>) {
        self.formal_paras = formal_paras;
    }

    pub(crate) fn set_func_body(&mut self, func_body: ASTNode<FuncBody>) {
        self.func_body = func_body;
    }
}

#[derive(Visualizable, Default)]
pub struct PropertyMemberDecl {}

#[derive(Visualizable, Default)]
pub struct IndexMemberDecl {}
