use super::{exp::Exp, identifier::Identifier};
use crate::ast::AstGraph;
use crate::ast::{visulize::Visualizable, ASTNode};

#[derive(Visualizable, Default)]
pub struct FormalParas {}

#[derive(Visualizable)]
pub struct TypeParas {}

#[derive(Visualizable)]
pub struct TypeAnnotation {}

// pub enum ParaList {
//     RestPara(ASTNode<RestPara>),
//     Paras(ASTNode<Paras>),
// }
#[derive(Visualizable)]
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

// #[derive(Visualizable)]
pub struct RestPara {
    exp: Exp,
    type_annotation: Option<ASTNode<TypeAnnotation>>,
}

impl Visualizable for RestPara {
    fn draw(&self, id: usize, graph: &mut AstGraph) {
        todo!()
    }
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
