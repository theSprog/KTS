use crate::ast::{visulize::Visualizable, ASTNode, AST_GRAPH};

use super::{exp::Exp, identifier::Identifier};

pub struct FormalParas {}
impl Visualizable for FormalParas {
    fn draw(&self, id: usize) {
        todo!()
    }
}

pub struct TypeParas {}
impl Visualizable for TypeParas {
    fn draw(&self, id: usize) {
        todo!()
    }
}

pub struct TypeAnnotation {}
impl Visualizable for TypeAnnotation {
    fn draw(&self, id: usize) {
        todo!()
    }
}

// pub enum ParaList {
//     RestPara(ASTNode<RestPara>),
//     Paras(ASTNode<Paras>),
// }

pub struct ParaList {
    paras: Vec<ASTNode<Para>>,
    rest_para: Option<ASTNode<RestPara>>,
}

impl ParaList {
    pub fn new() -> Self {
        ParaList {
            paras: Vec::new(),
            rest_para: None,
        }
    }
    pub(crate) fn push_para(&mut self, para: ASTNode<Para>) {
        self.paras.push(para);
    }

    pub(crate) fn set_rest_para(&mut self, rest_para: ASTNode<RestPara>) {
        self.rest_para = Some(rest_para);
    }
}

impl Visualizable for ParaList {
    fn draw(&self, id: usize) {
        AST_GRAPH::put_node(id, "ParaList");
        for para in &self.paras {
            AST_GRAPH::put_edge(id, para.id);
            para.draw();
        }
        if let Some(rest_para) = &self.rest_para {
            AST_GRAPH::put_edge(id, rest_para.id);
            rest_para.draw();
        }
    }
}

pub struct RestPara {
    exp: Exp,
    type_annotation: Option<TypeAnnotation>,
}
impl Visualizable for RestPara {
    fn draw(&self, id: usize) {
        AST_GRAPH::put_node(id, "RestPara");
    }
}

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

impl Visualizable for Para {
    fn draw(&self, id: usize) {
        AST_GRAPH::put_node(id, "Para");
        AST_GRAPH::put_edge(id, self.para_name.id);
        self.para_name.draw();
    }
}
