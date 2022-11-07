use crate::ast::*;

use super::stat::Stat;

#[derive(Visualizable)]
pub struct SourceElements {
    source_elements: Vec<ASTNode<Stat>>,
}

impl Default for SourceElements {
    fn default() -> Self {
        Self {
            source_elements: Vec::new(),
        }
    }
}

impl SourceElements {
    pub(crate) fn push_stat(&mut self, stat: ASTNode<Stat>) {
        self.source_elements.push(stat);
    }
}

// #[derive(Visualizable, Default)]
// pub struct SourceElement {
//     stat: ASTNode<Stat>,
// }
// impl SourceElement {
//     pub(crate) fn set_stat(&mut self, stat: ASTNode<Stat>) {
//         self.stat = stat
//     }
// }
