use super::decl::ObjectType;
use super::identifier::Identifier;
use super::parameter::ParaList;
use crate::ast::ASTNode;
use crate::ast::AstGraph;
use crate::ast::NodeInfo;
use crate::ast::Visualizable;

#[derive(Visualizable, Default)]
pub struct TypeRef {
    type_name: ASTNode<Identifier>,
    type_generic: Option<ASTNode<TypeGeneric>>,
}
impl TypeRef {
    pub(crate) fn set_type_name(&mut self, type_name: ASTNode<Identifier>) {
        self.type_name = type_name;
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

#[derive(Visualizable, Default)]
pub struct TypeGeneric {
    type_args: Vec<ASTNode<TypeArg>>,
}
impl TypeGeneric {
    pub(crate) fn push_type_arg(&mut self, type_arg: ASTNode<TypeArg>) {
        self.type_args.push(type_arg);
    }
}

#[derive(Visualizable, Default)]
pub struct TypeArgs {
    type_args: Vec<ASTNode<TypeArg>>,
}

#[derive(Visualizable)]
pub struct TypeArg {
    type_arg: ASTNode<Type>,
}

#[derive(Visualizable)]
pub enum Type {
    PrimaryType(ASTNode<PrimaryType>),
    FunctionType(ASTNode<FunctionType>),
}

#[derive(Visualizable)]
pub enum PrimaryType {
    PredefinedType(ASTNode<PredefinedType>),
    TypeRef(ASTNode<TypeRef>),
    ArrayPredefinedType(ASTNode<ArrayPredefinedType>),
    ArrayTypeRef(ASTNode<ArrayTypeRef>),
    TupleType(ASTNode<TupleElementTypes>),
    ObjectType(ASTNode<ObjectType>),
}

#[derive(Visualizable)]
pub struct FunctionType {
    para_list: Option<ASTNode<ParaList>>,
    type_: ASTNode<Type>,
}
impl FunctionType {
    pub(crate) fn new(para_list: Option<ASTNode<ParaList>>, type_: ASTNode<Type>) -> Self {
        Self { para_list, type_ }
    }
}

#[derive(Visualizable)]
pub struct ArrayPredefinedType {
    predefined_type: ASTNode<PredefinedType>,
}
impl ArrayPredefinedType {
    pub(crate) fn new(predefined_type: ASTNode<PredefinedType>) -> Self {
        Self { predefined_type }
    }
}
// impl Visualizable for ArrayPredefinedType {
//     fn draw(&self, self_id: usize, graph: &mut AstGraph) {
//         graph.put_node(self_id, "ArrayPredefinedType");
//         self.predefined_type.draw(self_id, graph);
//     }
// }

#[derive(Visualizable)]
pub struct ArrayTypeRef {
    array_type_ref: ASTNode<TypeRef>,
}
impl ArrayTypeRef {
    pub(crate) fn new(type_ref: ASTNode<TypeRef>) -> Self {
        Self {
            array_type_ref: type_ref,
        }
    }
}

#[derive(Default)]
pub enum PredefinedType {
    #[default]
    Any,
    Number,
    Boolean,
    String,
    Symbol,
    Void,
}

impl Visualizable for PredefinedType {
    fn draw(&self, self_info: NodeInfo, graph: &mut AstGraph) {
        match self {
            PredefinedType::Any => graph.put_node(self_info, "any"),
            PredefinedType::Number => graph.put_node(self_info, "number"),
            PredefinedType::Boolean => graph.put_node(self_info, "boolean"),
            PredefinedType::String => graph.put_node(self_info, "string"),
            PredefinedType::Symbol => graph.put_node(self_info, "symbol"),
            PredefinedType::Void => graph.put_node(self_info, "void"),
        }
    }
}

#[derive(Visualizable)]
pub struct TupleElementTypes {
    tuple_element_types: Vec<ASTNode<TupleElement>>,
}

#[derive(Visualizable)]
pub struct TupleElement {
    tuple_element: ASTNode<Type>,
}
