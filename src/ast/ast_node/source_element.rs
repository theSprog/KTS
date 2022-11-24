use crate::ast::*;

use super::stat::Stat;

#[derive(Visualizable, Default)]
pub struct SourceElements {
    pub(crate) stats: Vec<ASTNode<Stat>>,
}

impl SourceElements {
    pub(crate) fn push_stat(&mut self, stat: ASTNode<Stat>) {
        self.stats.push(stat);
    }
}
