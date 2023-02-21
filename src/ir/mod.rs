use crate::ast::AST;

pub struct IR {}
impl IR {
    pub(crate) fn gen_ir(ast: AST, ir_kind: IRKind) -> () {
        todo!()
    }
}

pub enum IRKind {
    LLVM,
    Own,
}
