use crate::ast::ASTNode;
use crate::ast::AstGraph;
use crate::ast::Visualizable;

use super::identifier::Identifier;

#[derive(Visualizable, Default)]
pub struct TypeRef {
    type_name: ASTNode<Identifier>,
    type_generic: Option<ASTNode<TypeGeneric>>,
}
impl TypeRef {
    pub(crate) fn set_type_name(&mut self, type_name: &str) {
        self.type_name = ASTNode::new(Identifier::new(type_name));
    }

    pub(crate) fn set_type_generic(&mut self, type_generic: ASTNode<TypeGeneric>) {
        self.type_generic = Some(type_generic);
    }
}

#[derive(Visualizable, Default)]
pub struct TypeRefs {
    type_refs: Vec<ASTNode<TypeRef>>,
}
impl TypeRefs {
    pub(crate) fn push(&mut self, type_ref: ASTNode<TypeRef>) {
        self.type_refs.push(type_ref);
    }
}

#[derive(Visualizable)]
pub struct TypeGeneric {
    type_args: Vec<ASTNode<TypeArg>>,
}

#[derive(Visualizable)]
pub struct TypeArg {
    type_arg: ASTNode<Type>,
}

// #[derive(Visualizable)]
pub enum Type {}

impl Visualizable for Type {
    fn draw(&self, id: usize, graph: &mut AstGraph) {
        todo!()
    }
}
