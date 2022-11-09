use crate::ast::*;

use super::stat::Stat;

#[derive(Visualizable, Default)]
pub struct SourceElements {
    source_elements: Vec<ASTNode<Stat>>,
}

impl SourceElements {
    pub(crate) fn push_stat(&mut self, stat: ASTNode<Stat>) {
        self.source_elements.push(stat);
    }
}
