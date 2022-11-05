use crate::lexer::KEYWORD;

use crate::{ast::*, lexer::token_kind::KeyWordKind};

use super::stat::Stat;

#[derive(Visualizable)]
pub struct SourceElements {
    source_elements: Vec<ASTNode<SourceElement>>,
}

impl SourceElements {
    pub(crate) fn new() -> SourceElements {
        Self {
            source_elements: Vec::new(),
        }
    }

    pub(crate) fn push_source_element(&mut self, source_element: ASTNode<SourceElement>) {
        self.source_elements.push(source_element);
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.source_elements.is_empty()
    }
}

#[derive(Visualizable)]
pub struct SourceElement {
    stat: ASTNode<Stat>,
}
impl SourceElement {
    pub(crate) fn new() -> Self {
        Self {
            stat: Default::default(),
        }
    }

    pub(crate) fn set_stat(&mut self, stat: ASTNode<Stat>) {
        self.stat = stat
    }
}
