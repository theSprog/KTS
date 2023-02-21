pub mod error;
pub mod eval_exp;
pub mod eval_obj;

use std::fmt::Debug;

use self::{error::EvalError, eval_obj::EvalObj};
use crate::ast::{
    self,
    ast_node::{
        exp::{Exp, ExpSeq},
        program::Program,
        source_element::SourceElements,
        stat::Stat,
    },
    ASTNode, AST,
};

pub(super) struct Eval {}

impl Eval {
    pub(super) fn walk(ast: &AST) -> Result<EvalObj, EvalError> {
        EvalError::set_filename(&ast.filename);
        let node = ast.get_program_ref();
        let Program { source_elements } = node.ctx_ref();
        if let Some(source_elements) = source_elements {
            return Eval::walk_source_elements(source_elements);
        }
        Ok(EvalObj::NONE)
    }

    fn walk_source_elements(
        source_elements: &ASTNode<SourceElements>,
    ) -> Result<EvalObj, EvalError> {
        let SourceElements { stats } = source_elements.ctx_ref();
        let mut res = EvalObj::NONE;
        for stat in stats {
            // unimplemented!()
            res = Eval::walk_stat(stat)?;
        }
        Ok(res)
    }

    fn walk_stat(stat: &ASTNode<Stat>) -> Result<EvalObj, EvalError> {
        match stat.ctx_ref() {
            Stat::ImportStat(import_stat) => todo!(),
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
            Stat::ExpStat(exp_seq) => Eval::walk_exp_stat(exp_seq),

            Stat::Unknown(_) => unreachable!(),
        }
    }

    fn walk_exp_stat(exp_seq: &ExpSeq) -> Result<EvalObj, EvalError> {
        let exps = exp_seq.get_exps();
        let mut res = EvalObj::NONE;
        for exp in exps {
            res = Eval::walk_exp(exp.ctx_ref())?;
        }
        Ok(res)
    }

    fn walk_exp(exp: &Exp) -> Result<EvalObj, EvalError> {
        Eval::eval_exp(exp)
    }
}
