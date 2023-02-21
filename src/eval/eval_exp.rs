use crate::ast::ast_node::{
    exp::{BinaryExp, Exp, GroupExp, Op},
    identifier, literal,
};

use super::{error::EvalError, eval_obj::EvalObj, Eval};

impl Eval {
    pub(super) fn eval_exp(exp: &Exp) -> Result<EvalObj, EvalError> {
        match exp {
            Exp::UnaryExp(_) => todo!(),
            Exp::BinaryExp(binary) => {
                let BinaryExp { left, op, right } = binary;
                let left = Eval::eval_exp(left.ctx_ref())?;
                let right = Eval::eval_exp(right.ctx_ref())?;
                match op {
                    Op::PostInc => todo!(),
                    Op::PostDec => todo!(),
                    Op::PreInc => todo!(),
                    Op::PreDec => todo!(),
                    Op::UnaryPlus => todo!(),
                    Op::UnaryMinus => todo!(),
                    Op::BitNot => todo!(),
                    Op::Not => todo!(),
                    Op::New => todo!(),
                    Op::Delete => todo!(),
                    Op::Typeof => todo!(),
                    Op::Assign => todo!(),
                    Op::Dot => todo!(),
                    Op::Index => todo!(),
                    Op::Call => todo!(),
                    Op::Multiply => left * right,
                    Op::Divide => left / right,
                    Op::Mod => left % right,
                    Op::Plus => left + right,
                    Op::Minus => left - right,
                    Op::GreaterThanEquals => todo!(),
                    Op::MoreThan => todo!(),
                    Op::LessThanEquals => todo!(),
                    Op::LessThan => todo!(),
                    Op::BitAnd => todo!(),
                    Op::And => todo!(),
                    Op::BitAndAssign => todo!(),
                    Op::BitOr => todo!(),
                    Op::Or => todo!(),
                    Op::BitOrAssign => todo!(),
                    Op::Instanceof => todo!(),
                    Op::In => todo!(),
                    Op::As => todo!(),
                    Op::IdentityEquals => todo!(),
                    Op::IdentityNotEquals => todo!(),
                    Op::Equals => todo!(),
                    Op::NotEquals => todo!(),
                    Op::MultiplyAssign => todo!(),
                    Op::DivideAssign => todo!(),
                    Op::ModulusAssign => todo!(),
                    Op::PlusAssign => todo!(),
                    Op::MinusAssign => todo!(),
                    Op::RightShiftLogicalAssign => todo!(),
                    Op::RightShiftArithmeticAssign => todo!(),
                    Op::RightShiftLogical => todo!(),
                    Op::RightShiftArithmetic => todo!(),
                    Op::LeftShiftArithmeticAssign => todo!(),
                    Op::LeftShiftArithmetic => todo!(),
                    Op::BitXOr => todo!(),
                    Op::BitXorAssign => todo!(),
                    Op::QuestionMark => todo!(),
                    Op::Colon => todo!(),
                }
            }
            Exp::TernaryExp(_) => todo!(),
            Exp::AssignExp(_) => todo!(),
            Exp::GroupExp(GroupExp {
                left_paren,
                exp,
                right_paren,
            }) => Eval::eval_exp(exp.ctx_ref()),
            Exp::ArgsExp(_) => todo!(),
            Exp::FunctionExp(_) => todo!(),
            Exp::ClassExp(_) => todo!(),
            Exp::ArrowFuncExp(_) => todo!(),
            Exp::NewExp(_) => todo!(),
            Exp::CastExp(_) => todo!(),
            Exp::Literal(literal) => Ok(match literal {
                literal::Literal::Number(number) => EvalObj::Number(*number),
                literal::Literal::Integer(integer) => EvalObj::Integer(*integer),
                literal::Literal::String(str) => todo!(),
                literal::Literal::Boolean(bool) => todo!(),
                literal::Literal::Null => EvalObj::NONE,
            }),
            Exp::This(_) => todo!(),
            Exp::Super(_) => todo!(),
            Exp::Identifier(identifier) => todo!(),
            Exp::ArrayExp(_) => todo!(),
        }
    }
}
