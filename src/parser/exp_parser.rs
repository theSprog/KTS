use crate::{
    ast::{
        ast_node::{decl::ArrowFuncExpDecl, exp::*, identifier::Identifier},
        ASTNode,
    },
    lexer::token_kind::{KeyWordKind, TokenKind},
};

use super::{error::ParserError, Parser};

impl Parser {
    pub(super) fn parse_exp(&mut self) -> Result<ASTNode<Exp>, ParserError> {
        let mut left = self.parse_single_exp()?;
        if self.is_assign_op() {
            let assign_op = self.extract_op()?;
            let right = self.parse_exp()?;
            left = ASTNode::new(Exp::AssignExp(ASTNode::new(AssignExp::new(
                left, assign_op, right,
            ))));
        }

        Ok(left)
    }

    /*
    不包含赋值语句的表达式
    single_exp:
        primary
        | single_exp % single_exp
        | single_exp / single_exp
        | single_exp * single_exp

        | single_exp + single_exp
        | single_exp - single_exp

        | single_exp << single_exp
        | single_exp >> single_exp
        | single_exp >>> single_exp

        | single_exp < single_exp
        | single_exp <= single_exp
        | single_exp > single_exp
        | single_exp >= single_exp
        | single_exp in single_exp
        | single_exp instanceof single_exp
        | single_exp as single_exp

        | single_exp == single_exp
        | single_exp != single_exp
        | single_exp === single_exp
        | single_exp !== single_exp

        | single_exp & single_exp
        | single_exp ^ single_exp
        | single_exp | single_exp
        | single_exp && single_exp
        | single_exp || single_exp
        | single_exp ? single_exp : single_exp   // 三元表达式
    */
    fn parse_single_exp(&mut self) -> Result<ASTNode<Exp>, ParserError> {
        let mut exp_stack = Vec::new();
        let mut op_stack = Vec::new();

        loop {
            let unary_exp = self.parse_unary_exp()?;
            exp_stack.push(unary_exp);
            if !self.is_single_exp_op() {
                break;
            }
            // 由于不是 single_exp_op 的都 break 了，此处提取出来的必是 single_exp_op
            let op = self.extract_op()?;
            self.push_op(&mut op_stack, &mut exp_stack, op)?;
        }
        self.extract_exp_from_stack(op_stack, exp_stack)
    }

    /*
    unary: base | prefixOp base | base postfixOp ;
    */
    fn parse_unary_exp(&mut self) -> Result<ASTNode<Exp>, ParserError> {
        let prefix = self.extract_prefix_op();
        let base_exp = self.parse_base_exp()?;
        let postfix = self.extract_postfix_op();

        if prefix.is_none() && postfix.is_none() {
            return Ok(base_exp);
        }

        if let Some(fix_op) = prefix.xor(postfix) {
            Ok(ASTNode::new(Exp::UnaryExp(ASTNode::new(UnaryExp::new(
                fix_op, base_exp,
            )))))
        } else {
            Err(self.report_error("just supports either postfix or prefix"))
        }
    }

    // . [] ()
    fn parse_base_exp(&mut self) -> Result<ASTNode<Exp>, ParserError> {
        let mut exp_stack = Vec::new();
        let mut op_stack = Vec::new();

        let atom_exp = self.parse_atom_exp()?;
        exp_stack.push(atom_exp);

        loop {
            if self.kind_is(TokenKind::LeftParen) {
                let args_exp;
                self.eat(TokenKind::LeftParen)?;
                // 函数调用有可能无参数
                if self.kind_is(TokenKind::RightParen) {
                    args_exp = ASTNode::new(Exp::ArgsExp(ASTNode::new(ArgsExp::default())));
                } else {
                    args_exp = ASTNode::new(Exp::ArgsExp(ASTNode::new(ArgsExp::new(
                        self.parse_exp_seq()?,
                    ))));
                }
                self.eat(TokenKind::RightParen)?;

                self.push_op(&mut op_stack, &mut exp_stack, Op::Call)?;
                exp_stack.push(args_exp);
            }
            if self.kind_is(TokenKind::LeftBrace) {
                self.eat(TokenKind::LeftBrace)?;
                let index_exp = self.parse_exp()?;
                self.eat(TokenKind::RightBrace)?;

                self.push_op(&mut op_stack, &mut exp_stack, Op::Index)?;
                exp_stack.push(index_exp);
            } else if self.kind_is(TokenKind::Dot) {
                self.eat(TokenKind::Dot)?;
                let other_atom_exp = self.parse_atom_exp()?;

                self.push_op(&mut op_stack, &mut exp_stack, Op::Dot)?;
                exp_stack.push(other_atom_exp);
            } else {
                break;
            }
        }

        self.extract_exp_from_stack(op_stack, exp_stack)
    }

    fn parse_atom_exp(&mut self) -> Result<ASTNode<Exp>, ParserError> {
        match self.peek_kind() {
            TokenKind::Identifier => match self.look_ahead() {
                // 如果是 a => ...
                TokenKind::ARROW => Ok(ASTNode::new(Exp::ArrowFuncExp(self.parse_arrow_func()?))),

                _ => Ok(ASTNode::new(Exp::Identifier(ASTNode::new(
                    Identifier::new(&self.extact_identifier()?),
                )))),
            },

            _ if self.is_literal() => Ok(ASTNode::new(Exp::Literal(ASTNode::new(
                self.extact_literal()?,
            )))),

            TokenKind::KeyWord(KeyWordKind::This) => {
                self.eat(TokenKind::KeyWord(KeyWordKind::This))?;
                Ok(ASTNode::new(Exp::This(ASTNode::new(KeyWordKind::This))))
            }

            TokenKind::KeyWord(KeyWordKind::Super) => {
                self.eat(TokenKind::KeyWord(KeyWordKind::Super))?;
                Ok(ASTNode::new(Exp::Super(ASTNode::new(KeyWordKind::Super))))
            }

            // parse [...]
            TokenKind::LeftBrace => {
                let mut array_exp = ArrayExp::default();

                self.eat(TokenKind::LeftBrace)?;
                if !self.kind_is(TokenKind::RightBrace) {
                    loop {
                        array_exp.push_element(self.parse_exp()?);
                        if self.kind_is(TokenKind::Comma) {
                            self.eat(TokenKind::Comma)?;
                        } else {
                            break;
                        }
                    }
                }
                self.eat(TokenKind::RightBrace)?;

                Ok(ASTNode::new(Exp::ArrayExp(ASTNode::new(array_exp))))
            }

            // parse (...)
            TokenKind::LeftParen => {
                // 先尝试是否是 (...) => ... 箭头函数
                match self.try_to(Parser::parse_arrow_func) {
                    Some(arrow_func) => Ok(ASTNode::new(Exp::ArrowFuncExp(arrow_func))),

                    // 如果不是,则说明是单个 group
                    None => Ok(ASTNode::new(Exp::GroupExp(self.parse_group_exp()?))),
                }
            }

            TokenKind::KeyWord(KeyWordKind::Function) => {
                Ok(ASTNode::new(Exp::FunctionExp(self.parse_func_exp_decl()?)))
            }

            TokenKind::KeyWord(KeyWordKind::New) => {
                Ok(ASTNode::new(Exp::NewExp(self.parse_new_exp_decl()?)))
            }

            _ => Err(self.expect_error("exp", "expression")),
        }
    }

    fn parse_group_exp(&mut self) -> Result<ASTNode<GroupExp>, ParserError> {
        self.eat(TokenKind::LeftParen)?;
        let group = self.parse_exp()?;
        self.eat(TokenKind::RightParen)?;

        Ok(ASTNode::new(GroupExp::new(group)))
    }

    // New Identifier typeArguments? (' (exp (',' exp)*)? ')'
    fn parse_new_exp_decl(&mut self) -> Result<ASTNode<NewExp>, ParserError> {
        let mut new_exp = NewExp::default();
        self.eat(TokenKind::KeyWord(KeyWordKind::New))?;
        new_exp.set_class_name(&self.extact_identifier()?);

        if self.kind_is(TokenKind::LessThan) {
            new_exp.set_type_args(self.parse_type_args()?);
        }

        self.eat(TokenKind::LeftParen)?;
        if !self.kind_is(TokenKind::RightParen) {
            new_exp.set_args(self.parse_exp_seq()?);
        }
        self.eat(TokenKind::RightParen)?;

        Ok(ASTNode::new(new_exp))
    }

    fn extract_prefix_op(&mut self) -> Option<Op> {
        match self.peek_kind() {
            TokenKind::KeyWord(KeyWordKind::Delete) => Some(Op::Delete),
            TokenKind::KeyWord(KeyWordKind::Typeof) => Some(Op::Typeof),
            TokenKind::PlusPlus => Some(Op::PreInc),
            TokenKind::MinusMinus => Some(Op::PreDec),
            TokenKind::Plus => Some(Op::UnaryPlus),
            TokenKind::Minus => Some(Op::UnaryMinus),
            TokenKind::BitNot => Some(Op::BitNot),
            TokenKind::Not => Some(Op::Not),
            _ => None,
        }
        .map(|op| {
            self.index += 1;
            op
        })
    }

    fn extract_postfix_op(&mut self) -> Option<Op> {
        match self.peek_kind() {
            TokenKind::PlusPlus => Some(Op::PostInc),
            TokenKind::MinusMinus => Some(Op::PostDec),
            _ => None,
        }
        .map(|op| {
            self.index += 1;
            op
        })
    }

    fn extract_op(&mut self) -> Result<Op, ParserError> {
        match self.peek_kind() {
            TokenKind::Assign => Ok(Op::Assign),
            TokenKind::PlusAssign => Ok(Op::PlusAssign),
            TokenKind::MinusAssign => Ok(Op::MinusAssign),
            TokenKind::MultiplyAssign => Ok(Op::MultiplyAssign),
            TokenKind::DivideAssign => Ok(Op::DivideAssign),
            TokenKind::ModulusAssign => Ok(Op::ModulusAssign),
            TokenKind::BitAndAssign => Ok(Op::BitAndAssign),
            TokenKind::BitOrAssign => Ok(Op::BitOrAssign),
            TokenKind::BitXorAssign => Ok(Op::BitXorAssign),
            TokenKind::LeftShiftArithmeticAssign => Ok(Op::LeftShiftArithmeticAssign),
            TokenKind::RightShiftArithmeticAssign => Ok(Op::RightShiftArithmeticAssign),
            TokenKind::RightShiftLogicalAssign => Ok(Op::RightShiftLogicalAssign),

            // ? :
            TokenKind::QuestionMark => Ok(Op::QuestionMark),
            TokenKind::Colon => Ok(Op::Colon),

            TokenKind::Plus => Ok(Op::Plus),
            TokenKind::Minus => Ok(Op::Minus),
            TokenKind::Multiply => Ok(Op::Multiply),
            TokenKind::Divide => Ok(Op::Divide),

            TokenKind::Or => Ok(Op::Or),
            TokenKind::And => Ok(Op::And),
            TokenKind::BitOr => Ok(Op::BitOr),
            TokenKind::BitXOr => Ok(Op::BitXOr),
            TokenKind::BitAnd => Ok(Op::BitAnd),

            TokenKind::Equals => Ok(Op::Equals),
            TokenKind::NotEquals => Ok(Op::NotEquals),
            TokenKind::IdentityEquals => Ok(Op::IdentityEquals),
            TokenKind::IdentityNotEquals => Ok(Op::IdentityNotEquals),

            TokenKind::LessThan => Ok(Op::LessThan),
            TokenKind::LessThanEquals => Ok(Op::LessThanEquals),
            TokenKind::MoreThan => Ok(Op::MoreThan),
            TokenKind::GreaterThanEquals => Ok(Op::GreaterThanEquals),
            TokenKind::KeyWord(KeyWordKind::In) => Ok(Op::In),
            TokenKind::KeyWord(KeyWordKind::Instanceof) => Ok(Op::Instanceof),
            TokenKind::KeyWord(KeyWordKind::As) => Ok(Op::As),

            TokenKind::LeftShiftArithmetic => Ok(Op::LeftShiftArithmetic),
            TokenKind::RightShiftArithmetic => Ok(Op::RightShiftArithmetic),
            TokenKind::RightShiftLogical => Ok(Op::RightShiftLogical),

            _ => unreachable!(),
        }
        .map(|op| {
            self.index += 1;
            op
        })
    }

    fn extract_exp_from_stack(
        &mut self,
        mut op_stack: Vec<Op>,
        mut exp_stack: Vec<ASTNode<Exp>>,
    ) -> Result<ASTNode<Exp>, ParserError> {
        loop {
            if exp_stack.len() == 1 && op_stack.is_empty() {
                return Ok(exp_stack.pop().unwrap());
            } else {
                // op_stack is not empty
                self.climb(&mut op_stack, &mut exp_stack)?;
            }
        }
    }

    fn push_op(
        &mut self,
        op_stack: &mut Vec<Op>,
        exp_stack: &mut Vec<ASTNode<Exp>>,
        op: Op,
    ) -> Result<(), ParserError> {
        loop {
            if op_stack.is_empty() {
                op_stack.push(op);
                break;
            }

            // 如果优先级无法爬山
            if let Some(true) = op_stack.last().and_then(|top_op| Some(op.hold(top_op))) {
                op_stack.push(op);
                break;
            }

            // 如果优先级爬山
            self.climb(op_stack, exp_stack)?;
        }

        Ok(())
    }

    fn climb(
        &mut self,
        op_stack: &mut Vec<Op>,
        exp_stack: &mut Vec<ASTNode<Exp>>,
    ) -> Result<(), ParserError> {
        let op = op_stack.last().unwrap();

        if op.is_bin_op() {
            let op = op_stack.pop().unwrap();
            if let (Some(right), Some(left)) = (exp_stack.pop(), exp_stack.pop()) {
                let exp = Exp::BinaryExp(ASTNode::new(BinaryExp::new(left, op, right)));
                exp_stack.push(ASTNode::new(exp));
            } else {
                return Err(self.report_error(&format!(
                    "{:?} is binary operater but expression missing",
                    op
                )));
            }
        }
        // todo 处理前缀后缀
        else if op.is_unary_op() {
            unreachable!()
        } else if op.is_tenary_op() {
            if op_stack.len() < 2 {
                // 只有一个元素
                return Err(self.report_error(&format!(
                    "{:?} is tenary operater but another operater missing",
                    op
                )));
            }

            if exp_stack.len() < 3 {
                return Err(self.report_error(&format!(
                    "tenary operater need three expression but there is only {}",
                    exp_stack.len()
                )));
            }

            let op_colon = op_stack.pop().unwrap();
            assert_eq!(op_colon, Op::Colon);
            let op_question = op_stack.pop().unwrap();
            assert_eq!(op_question, Op::QuestionMark);

            let false_br = exp_stack.pop().unwrap();
            let true_br = exp_stack.pop().unwrap();
            let cond = exp_stack.pop().unwrap();

            let exp = Exp::TernaryExp(ASTNode::new(TernaryExp::new(cond, true_br, false_br)));
            exp_stack.push(ASTNode::new(exp));
        } else {
            unreachable!()
        }

        Ok(())
    }

    fn is_assign_op(&mut self) -> bool {
        match self.peek_kind() {
            TokenKind::Assign              // =
            |TokenKind::PlusAssign          // +=
            |TokenKind::MinusAssign         // -=
            |TokenKind::MultiplyAssign      // *=
            |TokenKind::DivideAssign        // /=
            |TokenKind::ModulusAssign       // %=
            |TokenKind::BitAndAssign        // &=
            |TokenKind::BitOrAssign         // |=
            |TokenKind::BitXorAssign       // ^=
            |TokenKind::LeftShiftArithmeticAssign     // <<=
            |TokenKind::RightShiftArithmeticAssign  // >>=
            |TokenKind::RightShiftLogicalAssign => true, // >>>=
            _ => false //
        }
    }

    fn is_single_exp_op(&mut self) -> bool {
        match self.peek_kind() {
            // ? :
            TokenKind::QuestionMark
            | TokenKind::Colon
            // + - *  /
            | TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Multiply
            | TokenKind::Divide

            // || && | ^ &
            | TokenKind::Or
            | TokenKind::And
            | TokenKind::BitOr
            | TokenKind::BitXOr
            | TokenKind::BitAnd

            | TokenKind::Equals
            | TokenKind::NotEquals
            | TokenKind::IdentityEquals
            | TokenKind::IdentityNotEquals

            | TokenKind::LessThan
            | TokenKind::LessThanEquals
            | TokenKind::MoreThan
            | TokenKind::GreaterThanEquals

            | TokenKind::KeyWord(KeyWordKind::In)
            | TokenKind::KeyWord(KeyWordKind::Instanceof)
            | TokenKind::KeyWord(KeyWordKind::As)

            | TokenKind::LeftShiftArithmetic
            | TokenKind::RightShiftArithmetic
            | TokenKind::RightShiftLogical => true,

            _ => false,
        }
    }
}
