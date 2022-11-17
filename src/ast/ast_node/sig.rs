use super::identifier::Identifier;
use super::parameter::*;
use super::type_::{PredefinedType, Type};
use crate::ast::AstGraph;
use crate::ast::{visulize::Visualizable, ASTNode};
use crate::lexer::token_kind::{KeyWordKind, TokenKind};

#[derive(Visualizable, Default)]
pub struct CallSig {
    type_paras: Option<ASTNode<TypeParas>>,
    para_list: ASTNode<ParaList>,
    type_annotation: Option<ASTNode<TypeAnnotation>>,
}

impl CallSig {
    pub(crate) fn set_type_paras(&mut self, type_paras: ASTNode<TypeParas>) {
        self.type_paras = Some(type_paras);
    }

    pub(crate) fn set_para_list(&mut self, para_list: ASTNode<ParaList>) {
        self.para_list = para_list;
    }

    pub(crate) fn set_type_annotation(&mut self, type_annotation: ASTNode<TypeAnnotation>) {
        self.type_annotation = Some(type_annotation);
    }
}

#[derive(Default, Visualizable)]
pub struct PropertySig {
    readonly: Option<ASTNode<KeyWordKind>>,
    property_name: ASTNode<Identifier>,
    question_mark: Option<ASTNode<TokenKind>>,
    type_annotation: Option<ASTNode<TypeAnnotation>>,
}

impl PropertySig {
    pub(crate) fn set_readonly(&mut self) {
        self.readonly = Some(ASTNode::new(KeyWordKind::ReadOnly));
    }

    pub(crate) fn set_property_name(&mut self, identifier: &str) {
        self.property_name = ASTNode::new(Identifier::new(identifier));
    }

    pub(crate) fn set_question_mark(&mut self) {
        self.question_mark = Some(ASTNode::new(TokenKind::QuestionMark));
    }

    pub(crate) fn set_type_annotation(&mut self, type_annotation: ASTNode<TypeAnnotation>) {
        self.type_annotation = Some(type_annotation);
    }
}

#[derive(Default, Visualizable)]
pub struct MethodSig {
    method_name: ASTNode<Identifier>,
    question_mark: Option<ASTNode<TokenKind>>,
    call_sig: ASTNode<CallSig>,
    type_: Option<ASTNode<Type>>,
}
impl MethodSig {
    pub(crate) fn set_method_name(&mut self, identifier: &str) {
        self.method_name = ASTNode::new(Identifier::new(identifier));
    }

    pub(crate) fn set_question_mark(&mut self) {
        self.question_mark = Some(ASTNode::new(TokenKind::QuestionMark));
    }

    pub(crate) fn set_call_sig(&mut self, call_sig: ASTNode<CallSig>) {
        self.call_sig = call_sig;
    }

    pub(crate) fn set_type(&mut self, type_: ASTNode<Type>) {
        self.type_ = Some(type_);
    }
}

#[derive(Visualizable)]
pub struct IndexSig {
    index_name: ASTNode<Identifier>,
    type_: Option<ASTNode<PredefinedType>>,
    type_annotation: ASTNode<TypeAnnotation>,
}
impl IndexSig {
    pub(crate) fn new(
        index_name: &str,
        type_: Option<ASTNode<PredefinedType>>,
        type_annotation: ASTNode<TypeAnnotation>,
    ) -> Self {
        Self {
            index_name: ASTNode::new(Identifier::new(index_name)),
            type_,
            type_annotation,
        }
    }
}

#[derive(Visualizable, Default)]
pub struct ConstructSig {
    type_paras: Option<ASTNode<TypeParas>>,
    para_list: Option<ASTNode<ParaList>>,
    type_annotation: Option<ASTNode<TypeAnnotation>>,
}
impl ConstructSig {
    pub(crate) fn set_type_paras(&mut self, type_paras: ASTNode<TypeParas>) {
        self.type_paras = Some(type_paras);
    }

    pub(crate) fn set_para_list(&mut self, para_list: ASTNode<ParaList>) {
        self.para_list = Some(para_list);
    }

    pub(crate) fn set_type_annotation(&mut self, type_annotation: ASTNode<TypeAnnotation>) {
        self.type_annotation = Some(type_annotation);
    }
}
