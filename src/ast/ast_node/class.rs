use crate::ast::ASTNode;
use crate::ast::AstGraph;
use crate::ast::Visualizable;
use crate::lexer::token_kind::KeyWordKind;
use crate::lexer::token_kind::TokenKind;

use super::body::FuncBody;
use super::decl::AbsDecl;
use super::decorator::Decorators;
use super::exp::Exp;
use super::identifier::Identifier;
use super::parameter::FormalParas;
use super::parameter::TypeAnnotation;
use super::type_ref::*;

pub enum AccessModifier {
    Public,
    Protected,
    Private,
}

impl Visualizable for AccessModifier {
    fn draw(&self, father_id: usize, graph: &mut AstGraph) {
        match self {
            AccessModifier::Public => {
                graph.put_node(father_id, "public");
            }
            AccessModifier::Protected => {
                graph.put_node(father_id, "protected");
            }
            AccessModifier::Private => {
                graph.put_node(father_id, "private");
            }
        }
    }
}

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
    property_base: ASTNode<PropertyBase>,
    identifier: ASTNode<Identifier>,
    question_mark: Option<ASTNode<TokenKind>>,
    type_annotation: Option<ASTNode<TypeAnnotation>>,
    initializer: Option<ASTNode<Exp>>,
}
impl PropertyDeclExp {
    pub(crate) fn set_property_base(&mut self, property_base: ASTNode<PropertyBase>) {
        self.property_base = property_base;
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

#[derive(Visualizable)]
pub struct MethodDeclExp {
    async_: Option<ASTNode<KeyWordKind>>,
}

#[derive(Visualizable)]
pub struct GetterSetterDeclExp {}

// #[derive(Visualizable, Default)]
// pub struct PropertyMemberDecl {
//     property_base: ASTNode<PropertyBase>,
//     decorators: Option<ASTNode<Decorators>>,
//     abstract_decl: Option<ASTNode<AbsDecl>>,
// }
// impl PropertyMemberDecl {
//     pub(crate) fn set_property_base(&mut self, property_base: ASTNode<PropertyBase>) {
//         self.property_base = property_base
//     }

//     pub(crate) fn set_parse_decorators(&mut self, decorators: ASTNode<Decorators>) {
//         todo!()
//     }

//     pub(crate) fn set_abstract(&mut self, abstract_decl: ASTNode<AbsDecl>) {
//         todo!()
//     }
// }

#[derive(Visualizable, Default)]
pub struct PropertyBase {
    access_modifier: Option<ASTNode<AccessModifier>>,
    static_: Option<ASTNode<KeyWordKind>>,
    readonly: Option<ASTNode<KeyWordKind>>,
}

impl PropertyBase {
    pub(crate) fn set_access_modifier(&mut self, access_modifier: ASTNode<AccessModifier>) {
        self.access_modifier = Some(access_modifier);
    }

    pub(crate) fn set_static(&mut self) {
        self.static_ = Some(ASTNode::new(KeyWordKind::Static));
    }

    pub(crate) fn set_readonly(&mut self) {
        self.readonly = Some(ASTNode::new(KeyWordKind::ReadOnly));
    }
}

#[derive(Visualizable, Default)]
pub struct IndexMemberDecl {}
