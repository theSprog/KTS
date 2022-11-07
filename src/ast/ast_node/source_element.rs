use crate::lexer::KEYWORD;

use crate::{ast::*, lexer::token_kind::KeyWordKind};

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

    pub(crate) fn is_empty(&self) -> bool {
        self.source_elements.is_empty()
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
