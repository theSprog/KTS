use super::type_::*;
use super::{exp::Exp, identifier::Identifier};
use crate::ast::AstGraph;
use crate::ast::{visulize::Visualizable, ASTNode};
use crate::lexer::token_kind::{KeyWordKind, TokenKind};

#[derive(Visualizable, Default)]
pub struct FormalParas {
    formal_paras: Vec<ASTNode<FormalPara>>,
    last_para_arg: Option<ASTNode<Identifier>>,
}

impl FormalParas {
    pub(crate) fn push_formal_para(&mut self, formal_para: ASTNode<FormalPara>) {
        self.formal_paras.push(formal_para);
    }

    pub(crate) fn set_last_para_arg(&mut self, last_para_arg: &str) {
        self.last_para_arg = Some(ASTNode::new(Identifier::new(last_para_arg)));
    }
}

#[derive(Visualizable, Default)]
pub struct FormalPara {
    decorator: Option<ASTNode<TokenKind>>,
    access_modifier: Option<ASTNode<KeyWordKind>>,
    identifier: ASTNode<Identifier>,
    question_mark: Option<ASTNode<TokenKind>>,
    type_annotation: Option<ASTNode<TypeAnnotation>>,
}

impl FormalPara {
    pub(crate) fn set_decorator(&mut self) {
        self.decorator = Some(ASTNode::new(TokenKind::At));
    }

    pub(crate) fn set_access_modifier(&mut self, access_modifier: KeyWordKind) {
        self.access_modifier = Some(ASTNode::new(access_modifier));
    }
    pub(crate) fn set_identifier(&mut self, ident_str: &str) {
        self.identifier = ASTNode::new(Identifier::new(ident_str));
    }

    pub(crate) fn set_question_mark(&mut self) {
        self.decorator = Some(ASTNode::new(TokenKind::QuestionMark));
    }

    pub(crate) fn set_type_annotation(&mut self, type_annotation: ASTNode<TypeAnnotation>) {
        self.type_annotation = Some(type_annotation);
    }
}

#[derive(Visualizable)]
pub struct TypeParas {}

#[derive(Visualizable)]
pub struct TypeAnnotation {
    type_annotation: ASTNode<Type>,
}
impl TypeAnnotation {
    pub(crate) fn new(type_: ASTNode<Type>) -> Self {
        Self {
            type_annotation: type_,
        }
    }
}

// pub enum ParaList {
//     RestPara(ASTNode<RestPara>),
//     Paras(ASTNode<Paras>),
// }
#[derive(Visualizable, Default)]
pub struct ParaList {
    paras: Vec<ASTNode<Para>>,
    rest_para: Option<ASTNode<RestPara>>,
}

impl ParaList {
    pub(crate) fn push_para(&mut self, para: ASTNode<Para>) {
        self.paras.push(para);
    }

    pub(crate) fn set_rest_para(&mut self, rest_para: ASTNode<RestPara>) {
        self.rest_para = Some(rest_para);
    }
}

#[derive(Visualizable)]
pub struct RestPara {
    exp: Exp,
    type_annotation: Option<ASTNode<TypeAnnotation>>,
}

#[derive(Visualizable)]
pub struct Para {
    para_name: ASTNode<Identifier>,
}

impl Para {
    pub fn new() -> Self {
        Self {
            para_name: Default::default(),
        }
    }

    pub(crate) fn set_para_name(&mut self, para_name: &str) {
        self.para_name = ASTNode::new(Identifier::new(para_name));
    }
}
