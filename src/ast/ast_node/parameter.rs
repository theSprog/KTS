use super::class::AccessModifier;
use super::decorator::Decorators;
use super::type_::*;
use super::{exp::Exp, identifier::Identifier};
use crate::ast::visulize::AstGraph;
use crate::ast::{visulize::Visualizable, ASTNode, NodeInfo};
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

    pub(crate) fn set_last_para_arg(&mut self, last_para_arg: ASTNode<Identifier>) {
        self.last_para_arg = Some(last_para_arg);
    }
}

#[derive(Visualizable, Default)]
pub struct FormalPara {
    decorator: Option<TokenKind>,
    access_modifier: Option<ASTNode<AccessModifier>>,
    identifier: ASTNode<Identifier>,
    question_mark: Option<TokenKind>,
    type_annotation: Option<ASTNode<TypeAnnotation>>,
}

impl FormalPara {
    pub(crate) fn set_decorator(&mut self) {
        self.decorator = Some(TokenKind::At);
    }

    pub(crate) fn set_access_modifier(&mut self, access_modifier: ASTNode<AccessModifier>) {
        self.access_modifier = Some(access_modifier);
    }

    pub(crate) fn set_identifier(&mut self, identifier: ASTNode<Identifier>) {
        self.identifier = identifier;
    }

    pub(crate) fn set_question_mark(&mut self) {
        self.decorator = Some(TokenKind::QuestionMark);
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

#[derive(Visualizable, Default)]
pub struct Para {
    decorators: Option<ASTNode<Decorators>>,
    access_modifier: Option<ASTNode<AccessModifier>>,
    para_name: ASTNode<Identifier>,
    question_mark: Option<TokenKind>,
    type_annotation: Option<ASTNode<TypeAnnotation>>,
    initializer: Option<ASTNode<Initializer>>,
}

impl Para {
    pub(crate) fn set_decorators(&mut self, decorators: ASTNode<Decorators>) {
        self.decorators = Some(decorators);
    }

    pub(crate) fn set_access_modifier(&mut self, access_modifier: ASTNode<AccessModifier>) {
        self.access_modifier = Some(access_modifier);
    }

    pub(crate) fn set_para_name(&mut self, para_name: ASTNode<Identifier>) {
        self.para_name = para_name;
    }

    pub(crate) fn set_question_mark(&mut self) {
        self.question_mark = Some(TokenKind::QuestionMark);
    }

    pub(crate) fn set_type_annotation(&mut self, type_annotation: ASTNode<TypeAnnotation>) {
        self.type_annotation = Some(type_annotation);
    }

    pub(crate) fn set_initializer(&mut self, initializer: ASTNode<Initializer>) {
        self.initializer = Some(initializer);
    }
}

#[derive(Visualizable)]
pub struct Initializer {
    exp: ASTNode<Exp>,
}
impl Initializer {
    pub(crate) fn new(exp: ASTNode<Exp>) -> Self {
        Self { exp }
    }
}
