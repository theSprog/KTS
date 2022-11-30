use crate::{
    ast::{
        ast_node::{program::Program, source_element::SourceElements, stat::Stat},
        visulize::Visualizable,
        ASTNode,
    },
    symbol::{env::Env, Symbol},
};

use super::error::SematicsError;

pub(super) struct SymbolWalker<'a> {
    env: &'a mut Env,
}

// 符号表必然活得比 SymbolWalker 长
impl<'a, 'b: 'a> SymbolWalker<'a> {
    pub(super) fn new(env: &'b mut Env) -> Self {
        Self { env }
    }

    pub(super) fn walk(&mut self, node: &ASTNode<Program>) -> Result<(), SematicsError> {
        let Program { source_elements } = node.ctx_ref();
        if let Some(source_elements) = source_elements {
            self.walk_source_elements(source_elements)?;
        }
        Ok(())
    }

    fn walk_source_elements(
        &mut self,
        source_elements: &ASTNode<SourceElements>,
    ) -> Result<(), SematicsError> {
        let SourceElements { stats } = source_elements.ctx_ref();
        for stat in stats {
            self.walk_stat(stat)?;
        }
        Ok(())
    }

    fn walk_stat(&mut self, stat: &ASTNode<Stat>) -> Result<(), SematicsError> {
        match stat.ctx_ref() {
            Stat::ImportStat(import_stat) => (),
            Stat::ExportStat(_) => todo!(),
            Stat::EmptyStat(_) => todo!(),
            Stat::Block(_) => todo!(),
            Stat::ClassDecl(_) => todo!(),
            Stat::InterfaceDecl(_) => todo!(),
            Stat::AbsDecl(_) => todo!(),
            Stat::NamespaceDecl(_) => todo!(),
            Stat::FuncDecl(_) => todo!(),
            Stat::VarStat(_) => todo!(),
            Stat::EnumStat(_) => todo!(),
            Stat::IfStat(_) => todo!(),
            Stat::IterStat(_) => todo!(),
            Stat::ContinueStat(_) => todo!(),
            Stat::BreakStat(_) => todo!(),
            Stat::ReturnStat(_) => todo!(),
            Stat::YieldStat(_) => todo!(),
            Stat::WithStat(_) => todo!(),
            Stat::SwitchStat(_) => todo!(),
            Stat::ThrowStat(_) => todo!(),
            Stat::TypeAliasStat(_) => todo!(),
            Stat::DebuggerStat(_) => todo!(),
            Stat::TryStat(_) => todo!(),
            Stat::FuncExpDecl(_) => todo!(),
            Stat::GenFuncDecl(_) => todo!(),
            Stat::LabelledStat(_) => todo!(),
            Stat::ExpStat(_) => todo!(),
            Stat::Unknown(_) => unreachable!(),
        }
        Ok(())
    }
}
