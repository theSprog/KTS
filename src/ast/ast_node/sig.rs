use super::parameter::*;
use crate::ast::AstGraph;
use crate::ast::{visulize::Visualizable, ASTNode};

#[derive(Default, Visualizable)]
pub struct CallSig {
    type_paras: Option<ASTNode<TypeParas>>,
    para_list: Option<ASTNode<ParaList>>,
    type_annotation: Option<ASTNode<TypeAnnotation>>,
}
impl CallSig {
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

#[derive(Default, Visualizable)]
pub struct PropertySig {}

#[derive(Default, Visualizable)]
pub struct MethodSig {}

#[derive(Visualizable, Default)]
pub struct IndexSig {}

#[derive(Visualizable, Default)]
pub struct ConstructSig {}
