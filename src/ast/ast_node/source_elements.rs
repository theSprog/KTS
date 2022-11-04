use crate::lexer::KEYWORD;

use crate::{ast::*, lexer::token_kind::KeyWordKind};

use super::stat::Stat;

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

impl Visualizable for SourceElements {
    fn draw(&self, id: usize) {
        AST_GRAPH::put_node(id, "SourceElements");
        self.source_elements.draw(id);
    }
}

pub struct SourceElement {
    export: Option<ASTNode<KeyWordKind>>,
    stat: ASTNode<Stat>,
}
impl SourceElement {
    pub(crate) fn new() -> Self {
        Self {
            export: None,
            stat: Default::default(),
        }
    }

    pub(crate) fn add_export(&mut self) {
        self.export = Some(ASTNode::new(KeyWordKind::Export));
    }

    pub(crate) fn set_stat(&mut self, stat: ASTNode<Stat>) {
        self.stat = stat
    }
}

impl Visualizable for SourceElement {
    fn draw(&self, id: usize) {
        AST_GRAPH::put_node(id, "SourceElement");

        self.export.draw(id);

        // 画出结点
        self.stat.draw();
        // 将其和本结点连接到一起
        AST_GRAPH::put_edge(id, self.stat.id);
    }
}
