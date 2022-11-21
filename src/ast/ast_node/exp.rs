use std::cmp::PartialOrd;
use std::collections::HashMap;

use super::decl::{ArrowFuncExpDecl, ClassExp, FuncExpDecl, NamespaceName};
use super::identifier::Identifier;
use super::literal::Literal;
use super::type_::{Type, TypeArgs};
use crate::ast::{Span, Visualizable, AST};

use crate::ast::{ASTNode, AstGraph, NodeInfo};
use crate::lexer::token_kind::{KeyWordKind, TokenKind};
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
    pub static ref OP_PRIORITY: HashMap<Op, (usize, usize)> = {
        let mut map = HashMap::new();
        // assign 是右结合的
        map.insert(Op::Assign, (21, 20));              // =
        map.insert(Op::PlusAssign, (21, 20));          // +=
        map.insert(Op::MinusAssign, (21, 20));         // -=
        map.insert(Op::MultiplyAssign, (21, 20));      // *=
        map.insert(Op::DivideAssign, (21, 20));        // /=
        map.insert(Op::ModulusAssign, (21, 20));       // %=
        map.insert(Op::BitAndAssign, (21, 20));        // &=
        map.insert(Op::BitOrAssign, (21, 20));         // |=
        map.insert(Op::BitXorAssign, (21, 20));       // ^=
        map.insert(Op::LeftShiftArithmeticAssign, (21, 20));     // <<=
        map.insert(Op::RightShiftArithmeticAssign, (21, 20));  // >>=
        map.insert(Op::RightShiftLogicalAssign, (21, 20)); // >>>=

        map.insert(Op::QuestionMark, (30, 31));            // ? :
        map.insert(Op::Colon, (30, 31));            // ? :

        map.insert(Op::Or, (40, 41));                        // ||
        map.insert(Op::And, (50, 51));                       // &&
        map.insert(Op::BitOr, (60, 61));                        // |
        map.insert(Op::BitXOr, (70, 71));                  // ^
        map.insert(Op::BitAnd, (80, 81));                    // &

        map.insert(Op::Equals, (90, 91));  //  ==
        map.insert(Op::NotEquals, (90, 91)); // !=
        map.insert(Op::IdentityEquals, (90, 91));   // ===
        map.insert(Op::IdentityNotEquals, (90, 91));   // !==

        map.insert(Op::LessThan, (100, 101));                        //<
        map.insert(Op::LessThanEquals, (100, 101));                // <=
        map.insert(Op::MoreThan, (100, 101));                   // >
        map.insert(Op::GreaterThanEquals, (100, 101));               // >=
        map.insert(Op::In, (100, 101));                   // in
        map.insert(Op::Instanceof, (100, 101));                 // instanceof
        map.insert(Op::As, (100, 101));                 // as


        map.insert(Op::LeftShiftArithmetic, (110, 111));                        // <<
        map.insert(Op::RightShiftArithmetic, (110, 111));                   // >>
        map.insert(Op::RightShiftLogical, (110, 111));                    // >>>

        map.insert(Op::Plus, (120, 121));                        //+
        map.insert(Op::Minus, (120, 121));                        //-

        map.insert(Op::Multiply, (130, 131));                        // *
        map.insert(Op::Divide, (130, 131));                        // /
        map.insert(Op::Mod, (130, 131));                        // %

        // 右结合
        map.insert(Op::Delete, (151, 150));                        // delete
        map.insert(Op::Typeof, (151, 150));                        // typeof
        map.insert(Op::PreInc, (151, 150));                         // ++(pre)
        map.insert(Op::PreDec, (151, 150));                         // --(pre)
        map.insert(Op::UnaryPlus, (151, 150));                        // +
        map.insert(Op::UnaryMinus, (151, 150));                        // -
        map.insert(Op::BitNot, (151, 150));                        // ~
        map.insert(Op::Not, (151, 150));                    // !

        map.insert(Op::PostInc, (160, 161));                        // ++(post)
        map.insert(Op::PostDec, (160, 161));                        // --(post)

        map.insert(Op::New, (170, 171));                        // new

        map.insert(Op::Dot, (180, 181));                        // .
        map.insert(Op::Index, (180, 181));                        // []
        map.insert(Op::Call, (180, 181));              //  ()

        map
    };
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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
    Call,   // ()

    Multiply, // *
    Divide,   // /
    Mod,      // %
    Plus,     // +
    Minus,    // -

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
    QuestionMark, // ?
    Colon,        // :
}
impl Op {
    pub(crate) fn hold(&self, top_op: &Op) -> bool {
        match (self, top_op) {
            // 三元表达式的特殊性: 只有 colon 压 colon 才不是攀升
            (Op::QuestionMark, Op::Colon)
            | (Op::Colon, Op::QuestionMark)
            | (Op::QuestionMark, Op::QuestionMark) => true,
            _ => self > top_op,
        }
    }

    pub(crate) fn is_bin_op(&self) -> bool {
        !self.is_unary_op() && !self.is_tenary_op()
    }

    pub(crate) fn is_tenary_op(&self) -> bool {
        match self {
            Op::QuestionMark | Op::Colon => true,
            _ => false,
        }
    }

    pub(crate) fn is_unary_op(&self) -> bool {
        match self {
            Op::PostInc
            | Op::PostDec
            | Op::PreInc
            | Op::PreDec
            | Op::UnaryPlus
            | Op::UnaryMinus
            | Op::BitNot
            | Op::Not
            | Op::New
            | Op::Delete
            | Op::Typeof => true,
            _ => false,
        }
    }
}

impl Visualizable for Op {
    fn draw(&self, self_info: NodeInfo, graph: &mut AstGraph) {
        match self {
            Op::PostInc => graph.put_node(self_info, "++(post)"),
            Op::PostDec => graph.put_node(self_info, "--(post)"),
            Op::PreInc => graph.put_node(self_info, "++(pre)"),
            Op::PreDec => graph.put_node(self_info, "--(pre)"),
            Op::UnaryPlus => graph.put_node(self_info, "+(unary)"),
            Op::UnaryMinus => graph.put_node(self_info, "-(unary)"),
            Op::BitNot => graph.put_node(self_info, "~"),
            Op::Not => graph.put_node(self_info, "!"),
            Op::New => graph.put_node(self_info, "new"),
            Op::Delete => graph.put_node(self_info, "delete"),
            Op::Typeof => graph.put_node(self_info, "typeof"),
            Op::Plus => graph.put_node(self_info, "+"),

            Op::Assign => graph.put_node(self_info, "="),
            Op::Dot => graph.put_node(self_info, "."),
            Op::Index => graph.put_node(self_info, "[]"),
            Op::Call => graph.put_node(self_info, "call"),
            Op::Multiply => graph.put_node(self_info, "*"),
            Op::Divide => graph.put_node(self_info, "/"),
            Op::Mod => graph.put_node(self_info, "%"),
            Op::Minus => graph.put_node(self_info, "-"),
            Op::GreaterThanEquals => graph.put_node(self_info, ">="),
            Op::MoreThan => graph.put_node(self_info, ">"),
            Op::LessThanEquals => graph.put_node(self_info, "<="),
            Op::LessThan => graph.put_node(self_info, "<"),
            Op::BitAnd => graph.put_node(self_info, "&"),
            Op::And => graph.put_node(self_info, "&&"),
            Op::BitAndAssign => graph.put_node(self_info, "&="),
            Op::BitOr => graph.put_node(self_info, "|"),
            Op::Or => graph.put_node(self_info, "||"),
            Op::BitOrAssign => graph.put_node(self_info, "|="),
            Op::Instanceof => graph.put_node(self_info, "instanceof"),
            Op::In => graph.put_node(self_info, "in"),
            Op::As => graph.put_node(self_info, "as"),
            Op::IdentityEquals => graph.put_node(self_info, "==="),
            Op::IdentityNotEquals => graph.put_node(self_info, "!=="),
            Op::Equals => graph.put_node(self_info, "=="),
            Op::NotEquals => graph.put_node(self_info, "!="),
            Op::MultiplyAssign => graph.put_node(self_info, "*="),
            Op::DivideAssign => graph.put_node(self_info, "/="),
            Op::ModulusAssign => graph.put_node(self_info, "%="),
            Op::PlusAssign => graph.put_node(self_info, "+="),
            Op::MinusAssign => graph.put_node(self_info, "-="),
            Op::RightShiftLogicalAssign => graph.put_node(self_info, ">>>="),
            Op::RightShiftArithmeticAssign => graph.put_node(self_info, ">>="),
            Op::RightShiftLogical => graph.put_node(self_info, ">>>"),
            Op::RightShiftArithmetic => graph.put_node(self_info, ">>"),
            Op::LeftShiftArithmeticAssign => graph.put_node(self_info, "<<="),
            Op::LeftShiftArithmetic => graph.put_node(self_info, "<<"),
            Op::BitXOr => graph.put_node(self_info, "^"),
            Op::BitXorAssign => graph.put_node(self_info, "^="),
            Op::QuestionMark => graph.put_node(self_info, "?"),
            Op::Colon => graph.put_node(self_info, ":"),
        }
    }
}

impl PartialOrd for Op {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let lhs = OP_PRIORITY.get(other).unwrap();
        let rhs = OP_PRIORITY.get(self).unwrap();

        if lhs.1 < rhs.0 {
            Some(std::cmp::Ordering::Greater)
        } else if lhs.1 > rhs.0 {
            Some(std::cmp::Ordering::Less)
        } else {
            // 即便是同一个的符号，其左右结合性也是不同的
            unreachable!()
        }
    }
}

#[derive(Visualizable)]
pub enum Exp {
    UnaryExp(UnaryExp),
    BinaryExp(BinaryExp),
    TernaryExp(TernaryExp),

    AssignExp(AssignExp),

    GroupExp(GroupExp),

    ArgsExp(ArgsExp),

    FunctionExp(FuncExpDecl),
    ClassExp(ClassExp),

    ArrowFuncExp(ArrowFuncExpDecl),

    NewExp(NewExp),
    CastExp(CastExp),

    // 单个字面量，如 1, "abc"
    Literal(Literal),

    // this
    This(KeyWordKind),
    // super
    Super(KeyWordKind),

    // 其他单个标识符, 如 a, something
    Identifier(Identifier),

    // 数组
    ArrayExp(ArrayExp),
}

pub struct UnaryExp {
    op: Op,
    exp: ASTNode<Exp>,
}

impl UnaryExp {
    pub fn new(op: Op, exp: ASTNode<Exp>) -> Self {
        Self { op, exp }
    }
}

// 因为要区分前置和后置，手动实现 Visualizable
impl Visualizable for UnaryExp {
    fn draw(&self, self_info: NodeInfo, graph: &mut AstGraph) {
        graph.put_node(self_info, "UnaryExp");

        match self.op {
            Op::PostDec | Op::PostInc => {
                self.exp.draw(self_info, graph);
                self.op.draw(self_info, graph);
            }
            _ => {
                self.op.draw(self_info, graph);
                self.exp.draw(self_info, graph);
            }
        }
    }
}

#[derive(Visualizable)]
pub struct BinaryExp {
    left: ASTNode<Exp>,
    op: Op,
    right: ASTNode<Exp>,
}

impl BinaryExp {
    pub fn new(left: ASTNode<Exp>, op: Op, right: ASTNode<Exp>) -> Self {
        Self { left, op, right }
    }
}

#[derive(Visualizable)]
pub struct AssignExp {
    left: ASTNode<Exp>,
    op: Op,
    right: ASTNode<Exp>,
}

impl AssignExp {
    pub fn new(left: ASTNode<Exp>, op: Op, right: ASTNode<Exp>) -> Self {
        Self { left, op, right }
    }
}

#[derive(Visualizable)]
pub struct TernaryExp {
    cond: ASTNode<Exp>,
    true_branche: ASTNode<Exp>,
    false_branche: ASTNode<Exp>,
}
impl TernaryExp {
    pub(crate) fn new(
        cond: ASTNode<Exp>,
        true_branche: ASTNode<Exp>,
        false_branche: ASTNode<Exp>,
    ) -> Self {
        Self {
            cond,
            true_branche,
            false_branche,
        }
    }
}

#[derive(Visualizable)]
pub struct GroupExp {
    left_paren: ASTNode<TokenKind>,
    exp: ASTNode<Exp>,
    right_paren: ASTNode<TokenKind>,
}
impl GroupExp {
    pub(crate) fn new(exp: ASTNode<Exp>) -> Self {
        Self {
            left_paren: ASTNode::new(TokenKind::LeftParen, Span::default()),
            exp,
            right_paren: ASTNode::new(TokenKind::RightParen, Span::default()),
        }
    }
}

// // #[derive(Visualizable)]
// type ArgsExp = ExpSeq;

#[derive(Visualizable, Default)]
pub struct ArgsExp {
    args: Vec<ASTNode<Exp>>,
}

impl ArgsExp {
    pub(crate) fn new(args: ASTNode<ExpSeq>) -> Self {
        Self {
            args: args.ctx().exps,
        }
    }
}

#[derive(Visualizable, Default)]
pub struct NewExp {
    class_name: ASTNode<NamespaceName>,
    type_args: Option<ASTNode<TypeArgs>>,
    args: ASTNode<ArgsExp>,
}

impl NewExp {
    pub(crate) fn set_class_name(&mut self, class_name: ASTNode<NamespaceName>) {
        self.class_name = class_name;
    }

    pub(crate) fn set_type_args(&mut self, type_args: ASTNode<TypeArgs>) {
        self.type_args = Some(type_args);
    }

    pub(crate) fn set_args(&mut self, args: ASTNode<ArgsExp>) {
        self.args = args;
    }
}

#[derive(Visualizable)]
pub struct CastExp {
    type_: ASTNode<Type>,
}
impl CastExp {
    pub(crate) fn new(type_: ASTNode<Type>) -> Self {
        Self { type_ }
    }
}

#[derive(Visualizable, Default)]
pub struct ArrayExp {
    array_elements: Vec<ASTNode<Exp>>,
}

impl ArrayExp {
    pub(crate) fn push_element(&mut self, array_element: ASTNode<Exp>) {
        self.array_elements.push(array_element);
    }
}
