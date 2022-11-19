use crate::ast::{ASTNode, AstGraph, NodeInfo, Visualizable};

use super::source_element::SourceElements;

// #[derive(Visualizable, Default)]
// pub struct FuncBody {
//     source_elements: Option<ASTNode<SourceElements>>,
// }
// impl FuncBody {
//     pub(crate) fn set_func_body(&mut self, source_elements: ASTNode<SourceElements>) {
//         self.source_elements = Some(source_elements);
//     }
// }

#[derive(Visualizable)]
pub struct TypeBody {}

#[derive(Visualizable)]
pub struct EnumBody {}
