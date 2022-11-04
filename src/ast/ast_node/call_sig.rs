use crate::ast::{visulize::Visualizable, ASTNode, AST_GRAPH};

use super::parameter::*;

#[derive(Default)]
pub struct CallSig {
    type_paras: Option<ASTNode<TypeParas>>,
    para_list: Option<ASTNode<ParaList>>,
    type_annotation: Option<ASTNode<TypeAnnotation>>,
}
impl CallSig {
    pub(crate) fn new() -> Self {
        Self {
            type_paras: None,
            para_list: None,
            type_annotation: None,
        }
    }

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

impl Visualizable for CallSig {
    fn draw(&self, id: usize) {
        AST_GRAPH::put_node(id, "CallSig");

        if let Some(type_paras) = &self.type_paras {
            AST_GRAPH::put_edge(id, type_paras.id);
            type_paras.draw();
        }
        if let Some(para_list) = &self.para_list {
            AST_GRAPH::put_edge(id, para_list.id);
            para_list.draw();
        }
        if let Some(type_annotation) = &self.type_annotation {
            AST_GRAPH::put_edge(id, type_annotation.id);
            type_annotation.draw();
        }
    }
}
