use crate::ast::visulize::Visualizable;
use crate::ast::{ASTNode, AstGraph};

use super::source_elements::SourceElements;

#[derive(Visualizable, Default)]
pub struct FuncBody {
    source_elements: Option<ASTNode<SourceElements>>,
}
impl FuncBody {
    pub(crate) fn set_func_body(&mut self, source_elements: ASTNode<SourceElements>) {
        self.source_elements = Some(source_elements);
    }
}

#[derive(Visualizable)]
pub struct TypeBody {}

#[derive(Visualizable)]
pub struct EnumBody {}
#[derive(Visualizable)]
pub struct ArrowFuncBody {}
