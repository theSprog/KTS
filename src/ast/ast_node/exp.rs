use std::collections::HashMap;

use super::identifier::Identifier;
use super::literal::Literal;
use super::unknown::Unknown;
use crate::ast::Visualizable;

use crate::ast::{ASTNode, AstGraph};
use crate::lexer::token_kind::KeyWordKind;
use lazy_static::lazy_static;

#[derive(Visualizable, Default)]
pub struct ExpSeq {
    exps: Vec<ASTNode<Exp>>,
}
impl ExpSeq {
    pub(crate) fn push_exp(&mut self, single_exp: ASTNode<Exp>) {
        self.exps.push(single_exp);
    }
}

lazy_static! {
    pub static ref OP_PRIORITY: HashMap<Op, usize> = {
        let mut map = HashMap::new();
        // assign 是右结合的
        map.insert(Op::Assign, 2);
        map.insert(Op::PlusAssign, 2);          // +=
        map.insert(Op::MinusAssign, 2);         // -=
        map.insert(Op::MultiplyAssign, 2);      // *=
        map.insert(Op::DivideAssign, 2);        // /=
        map.insert(Op::ModulusAssign, 2);       // %=
        map.insert(Op::BitAndAssign, 2);        // &=
        map.insert(Op::BitOrAssign, 2);         // |=
        map.insert(Op::BitXorAssign, 2);       // ^=
        map.insert(Op::LeftShiftArithmeticAssign, 2);     // <<=
        map.insert(Op::RightShiftArithmeticAssign, 2);  // >>=
        map.insert(Op::RightShiftLogicalAssign, 2); // >>>=

        map.insert(Op::QuestionMarkColon, 3);            // ? :

        map.insert(Op::Or, 4);                        // ||
        map.insert(Op::And, 5);                       // &&
        map.insert(Op::BitOr, 6);                        // |
        map.insert(Op::BitXOr, 7);                  // ^
        map.insert(Op::BitAnd, 8);                    // &

        map.insert(Op::Equals, 9);  //  ==
        map.insert(Op::NotEquals, 9); // !=
        map.insert(Op::IdentityEquals, 9);   // ===
        map.insert(Op::IdentityNotEquals, 9);   // !==

        map.insert(Op::LessThan, 10);                        //<
        map.insert(Op::LessThanEquals, 10);                // <=
        map.insert(Op::MoreThan, 10);                   // >
        map.insert(Op::GreaterThanEquals, 10);               // >=
        map.insert(Op::In, 10);                   // in
        map.insert(Op::Instanceof, 10);                 // instanceof
        map.insert(Op::As, 10);                 // as


        map.insert(Op::LeftShiftArithmetic, 11);                        // <<
        map.insert(Op::RightShiftArithmetic, 11);                   // >>
        map.insert(Op::RightShiftLogical, 11);                    // >>>

        map.insert(Op::Plus, 12);                        //+
        map.insert(Op::Minus, 12);                        //-

        map.insert(Op::Mul, 13);                        // *
        map.insert(Op::Div, 13);                        // /
        map.insert(Op::Mod, 13);                        // %

        map.insert(Op::Delete, 15);                        // delete
        map.insert(Op::Typeof, 15);                        // typeof
        map.insert(Op::PreInc, 15);                         // ++
        map.insert(Op::PreDec, 15);                         // --
        map.insert(Op::UnaryPlus, 15);                        // +
        map.insert(Op::UnaryMinus, 15);                        // -
        map.insert(Op::BitNot, 15);                        // ~
        map.insert(Op::Not, 15);                    // !

        map.insert(Op::PostInc, 16);                        // ++
        map.insert(Op::PostDec, 16);                        // --

        map.insert(Op::New, 17);                        // &

        map.insert(Op::Dot, 18);                        // .
        map.insert(Op::Index, 18);                        // []

        map
    };
}

#[derive(PartialEq, Eq, Hash)]
pub enum Op {
    PostInc,
    PostDec,
    PreInc,
    PreDec,
    UnaryPlus,
    UnaryMinus,
    BitNot,
    Not,
    New,
    Delete,
    Typeof,

    // ----------------------------------------------------------------------------------------
    // binary
    Assign, // =
    Dot,    // .
    Index,  // []

    Mul,   // *
    Div,   // /
    Mod,   // %
    Plus,  // +
    Minus, // -

    GreaterThanEquals, // >=
    MoreThan,          // >

    LessThanEquals, // <=
    LessThan,       // <

    BitAnd,       // &
    And,          // &&
    BitAndAssign, // &=
    BitOr,        // |
    Or,           // ||
    BitOrAssign,  // |=

    Instanceof, //  instanceof
    In,         // in
    As,         // as

    IdentityEquals,    // ===
    IdentityNotEquals, // !==

    Equals,    // ==
    NotEquals, // !=

    MultiplyAssign, // *=
    DivideAssign,   // /=
    ModulusAssign,  // %=
    PlusAssign,     // +=
    MinusAssign,    // -=

    RightShiftLogicalAssign,    // >>>=
    RightShiftArithmeticAssign, // >>=
    RightShiftLogical,          // >>>
    RightShiftArithmetic,       // >>

    LeftShiftArithmeticAssign, // <<=
    LeftShiftArithmetic,       // <<

    BitXOr,       // ^
    BitXorAssign, // ^=

    // ----------------------------------------------------------------------------------------
    // ternary
    QuestionMarkColon, // ? :
}
impl Op {
    pub(crate) fn greater_than(&self, top_op: &Op) -> bool {
        OP_PRIORITY.get(self).unwrap() > OP_PRIORITY.get(top_op).unwrap()
    }
}

impl Visualizable for Op {
    fn draw(&self, self_id: usize, graph: &mut AstGraph) {
        match self {
            Op::PostInc => graph.put_node(self_id, "++(post)"),
            Op::PostDec => graph.put_node(self_id, "--(post)"),
            Op::PreInc => graph.put_node(self_id, "++(pre)"),
            Op::PreDec => graph.put_node(self_id, "--(pre)"),
            Op::UnaryPlus => graph.put_node(self_id, "+"),
            Op::UnaryMinus => graph.put_node(self_id, "-"),
            Op::BitNot => graph.put_node(self_id, "~"),
            Op::Not => graph.put_node(self_id, "!"),
            Op::New => graph.put_node(self_id, "new"),
            Op::Delete => graph.put_node(self_id, "delete"),
            Op::Typeof => graph.put_node(self_id, "typeof"),
            Op::Plus => graph.put_node(self_id, "+"),

            Op::Assign => todo!(),
            Op::Dot => todo!(),
            Op::Index => todo!(),
            Op::Mul => todo!(),
            Op::Div => todo!(),
            Op::Mod => todo!(),
            Op::Minus => todo!(),
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
            Op::QuestionMarkColon => todo!(),
        }
    }
}

#[derive(Visualizable)]
pub enum Exp {
    UnaryExp(ASTNode<UnaryExp>),
    BinaryExp(ASTNode<BinaryExp>),
    TernaryExp(ASTNode<TernaryExp>),

    // 单个字面量，如 1, "abc"
    Literal(ASTNode<Literal>),

    // this
    This(ASTNode<KeyWordKind>),
    // super, 由于宏的原因使用 Super_ 作标识
    Super_(ASTNode<KeyWordKind>),

    // 其他单个标识符, 如 a, something
    Identifier(ASTNode<Identifier>),
}

#[derive(Visualizable)]
pub struct UnaryExp {
    op: ASTNode<Op>,
    exp: ASTNode<Exp>,
}

#[derive(Visualizable)]
pub struct BinaryExp {
    left: ASTNode<Exp>,
    op: ASTNode<Op>,
    right: ASTNode<Exp>,
}

impl BinaryExp {
    pub fn new(left: ASTNode<Exp>, op: Op, right: ASTNode<Exp>) -> Self {
        Self {
            left,
            op: ASTNode::new(op),
            right,
        }
    }
}

#[derive(Visualizable)]
pub struct TernaryExp {
    cond: ASTNode<Exp>,
    true_branche: ASTNode<Exp>,
    false_branche: ASTNode<Exp>,
}
