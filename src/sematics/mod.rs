pub mod error;
mod symbol_walker;
mod type_check;

use crate::{
    ast::{
        ast_node::{program::Program, source_element::SourceElements, stat::Stat},
        visulize::Visualizable,
        ASTNode,
    },
    symbol::env::Env,
};

use self::{error::SematicsError, symbol_walker::SymbolWalker, type_check::TypeChecker};

pub(crate) struct SematicsWalker {}

/*
语义分析任务
1. 生成符号表
2. 类型检查
3. 变量使用前必须先定义
4. ...
*/
impl SematicsWalker {
    pub(crate) fn walk(program: &ASTNode<Program>) -> Result<Env, SematicsError> {
        let mut gloabel_env = Env::new_global();
        // 生成符号表
        SymbolWalker::new(&mut gloabel_env).walk(program);
        // 类型检查
        TypeChecker::check(&gloabel_env)?;
        Ok(gloabel_env)
    }
}
