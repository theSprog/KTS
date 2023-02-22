use crate::{
    ast::{
        ast_node::{
            decl::{ArrowFuncExpDecl, ClassExp},
            exp::*,
            identifier::Identifier,
        },
        ASTNode, Span,
    },
    lexer::token_kind::{KeyWordKind, TokenKind},
};

use super::{error::ParserError, Parser, ParseResult};

impl Parser {
    pub(super) fn parse_exp(&mut self) -> ParseResult<ASTNode<Exp>> {
        let begin = self.mark_begin();

        let mut left = self.parse_single_exp()?;
        if self.is_assign_op() {
            let assign_op = self.extract_op()?;
            let right = self.parse_exp()?;
            left = ASTNode::new(
                Exp::AssignExp(AssignExp::new(left, assign_op, right)),
                Span::new(begin, self.mark_end()),
            );
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
    fn parse_single_exp(&mut self) -> ParseResult<ASTNode<Exp>> {
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

            // 特殊的 as cast operator
            if op == Op::As {
                let type_ = self.parse_type()?;
                let span = type_.info.span; // span 实现了 copy
                let cast_exp = ASTNode::new(Exp::CastExp(CastExp::new(type_)), span);
                exp_stack.push(cast_exp);
                if !self.is_single_exp_op() {
                    break;
                }
            }
        }
        self.extract_exp_from_stack(op_stack, exp_stack)
    }

    /*
    unary: base | prefixOp base | base postfixOp ;
    */
    fn parse_unary_exp(&mut self) -> ParseResult<ASTNode<Exp>> {
        let begin = self.mark_begin();

        let prefix = self.extract_prefix_op();
        let base_exp = self.parse_base_exp()?;
        let postfix = self.extract_postfix_op();

        if prefix.is_none() && postfix.is_none() {
            return Ok(base_exp);
        }

        if let Some(op) = prefix.xor(postfix) {
            Ok(ASTNode::new(
                Exp::UnaryExp(UnaryExp::new(op, base_exp)),
                Span::new(begin, self.mark_end()),
            ))
        } else {
            Err(self.report_error("just supports either postfix or prefix"))
        }
    }

    // . [] ()
    fn parse_base_exp(&mut self) -> ParseResult<ASTNode<Exp>> {
        let begin = self.mark_begin();

        let mut exp_stack = Vec::new();
        let mut op_stack = Vec::new();

        let atom_exp = self.parse_atom_exp()?;
        exp_stack.push(atom_exp);

        loop {
            if self.kind_is(TokenKind::LeftParen) {
                self.eat(TokenKind::LeftParen)?;
                // 函数调用有可能无参数
                let args_exp = if self.kind_is(TokenKind::RightParen) {
                    ASTNode::new(
                        Exp::ArgsExp(ArgsExp::default()),
                        Span::new(begin, self.mark_end()),
                    )
                } else {
                    ASTNode::new(
                        Exp::ArgsExp(ArgsExp::new(self.parse_exp_seq()?)),
                        Span::new(begin, self.mark_end()),
                    )
                };
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

    fn parse_atom_exp(&mut self) -> ParseResult<ASTNode<Exp>> {
        let begin = self.mark_begin();

        match self.peek_kind() {
            TokenKind::Identifier => match self.next_kind() {
                // 如果是 a => ...
                TokenKind::Arrow => Ok(ASTNode::new(
                    Exp::ArrowFuncExp(self.parse_arrow_func()?.ctx()),
                    Span::new(begin, self.mark_end()),
                )),

                _ => Ok(ASTNode::new(
                    Exp::Identifier(self.parse_identifier()?.ctx()),
                    Span::new(begin, self.mark_end()),
                )),
            },

            _ if self.is_literal() => Ok(ASTNode::new(
                Exp::Literal(self.extact_literal()?),
                Span::new(begin, self.mark_end()),
            )),

            // Class Identifier? classTail
            TokenKind::KeyWord(KeyWordKind::Class) => {
                self.eat(TokenKind::KeyWord(KeyWordKind::Class))?;

                let mut class_exp = ClassExp::default();
                if self.kind_is(TokenKind::Identifier) {
                    class_exp.set_class_name(self.parse_identifier()?);
                }

                class_exp.set_class_tail(self.parse_class_tail()?);

                Ok(ASTNode::new(
                    Exp::ClassExp(class_exp),
                    Span::new(begin, self.mark_end()),
                ))
            }

            // -------------------------------------------------------------------
            TokenKind::KeyWord(KeyWordKind::This) => {
                self.forward();
                Ok(ASTNode::new(
                    Exp::This(KeyWordKind::This),
                    Span::new(begin, self.mark_end()),
                ))
            }

            // ----------------------------------------------------------------
            TokenKind::KeyWord(KeyWordKind::Super) => {
                self.forward();
                Ok(ASTNode::new(
                    Exp::Super(KeyWordKind::Super),
                    Span::new(begin, self.mark_end()),
                ))
            }

            // ----------------------------------------------------------------
            // parse [...]
            TokenKind::LeftBrace => {
                let mut array_exp = ArrayExp::default();

                self.eat(TokenKind::LeftBrace)?;
                loop {
                    if self.kind_is(TokenKind::RightBrace) {
                        break;
                    }
                    array_exp.push_element(self.parse_exp()?);
                    if self.kind_is(TokenKind::Comma) {
                        self.forward();
                    }
                }
                self.eat(TokenKind::RightBrace)?;

                Ok(ASTNode::new(
                    Exp::ArrayExp(array_exp),
                    Span::new(begin, self.mark_end()),
                ))
            }

            TokenKind::LeftBracket => Err(self.unsupported_error("objectLiteral expression")),

            // ---------------------------------------------------------------
            // parse (...)
            TokenKind::LeftParen => {
                // 先尝试是否是 (...) => ... 箭头函数
                match self.try_to(Parser::parse_arrow_func) {
                    Some(arrow_func) => Ok(ASTNode::new(
                        Exp::ArrowFuncExp(arrow_func.ctx()),
                        Span::new(begin, self.mark_end()),
                    )),

                    // 如果不是,则说明是单个 group
                    None => Ok(ASTNode::new(
                        Exp::GroupExp(self.parse_group_exp()?.ctx()),
                        Span::new(begin, self.mark_end()),
                    )),
                }
            }

            // ----------------------------------------------------------------
            // TokenKind::KeyWord(KeyWordKind::Function) => Ok(ASTNode::new(
            //     Exp::FunctionExp(self.parse_func_exp_decl()?),
            //     Span::new(begin, self.mark_end()),
            // )),
            TokenKind::KeyWord(KeyWordKind::Function) => Ok(ASTNode::new(
                Exp::FunctionExp(self.parse_func_exp_decl()?.ctx()),
                Span::new(begin, self.mark_end()),
            )),

            // ----------------------------------------------------------------
            TokenKind::KeyWord(KeyWordKind::New) => Ok(ASTNode::new(
                Exp::NewExp(self.parse_new_exp_decl()?.ctx()),
                Span::new(begin, self.mark_end()),
            )),

            _ => Err(self.expect_error("exp", "expression")),
        }
    }

    fn parse_group_exp(&mut self) -> ParseResult<ASTNode<GroupExp>> {
        let begin = self.mark_begin();

        self.eat(TokenKind::LeftParen)?;
        let group = self.parse_exp()?;
        self.eat(TokenKind::RightParen)?;

        Ok(ASTNode::new(
            GroupExp::new(group),
            Span::new(begin, self.mark_end()),
        ))
    }

    // New NamespaceName typeArguments? (' (exp (',' exp)*)? ')'
    fn parse_new_exp_decl(&mut self) -> ParseResult<ASTNode<NewExp>> {
        let begin = self.mark_begin();
        let mut new_exp = NewExp::default();
        self.eat(TokenKind::KeyWord(KeyWordKind::New))?;
        new_exp.set_class_name(self.parse_namespace_name()?);

        if self.kind_is(TokenKind::LessThan) {
            new_exp.set_type_args(self.parse_type_args()?);
        }

        self.eat(TokenKind::LeftParen)?;

        if self.kind_is(TokenKind::RightParen) {
            let empty_args = ASTNode::new(ArgsExp::default(), Span::new(begin, self.mark_end()));
            new_exp.set_args(empty_args)
        } else {
            let args_exp = ASTNode::new(
                ArgsExp::new(self.parse_exp_seq()?),
                Span::new(begin, self.mark_end()),
            );
            new_exp.set_args(args_exp)
        }

        self.eat(TokenKind::RightParen)?;

        Ok(ASTNode::new(new_exp, Span::new(begin, self.mark_end())))
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

    fn extract_op(&mut self) -> ParseResult<Op> {
        let op = match self.peek_kind() {
            TokenKind::Assign => Op::Assign,
            TokenKind::PlusAssign => Op::PlusAssign,
            TokenKind::MinusAssign => Op::MinusAssign,
            TokenKind::MultiplyAssign => Op::MultiplyAssign,
            TokenKind::DivideAssign => Op::DivideAssign,
            TokenKind::ModulusAssign => Op::ModulusAssign,
            TokenKind::BitAndAssign => Op::BitAndAssign,
            TokenKind::BitOrAssign => Op::BitOrAssign,
            TokenKind::BitXorAssign => Op::BitXorAssign,
            TokenKind::LeftShiftArithmeticAssign => Op::LeftShiftArithmeticAssign,
            TokenKind::RightShiftArithmeticAssign => Op::RightShiftArithmeticAssign,
            TokenKind::RightShiftLogicalAssign => Op::RightShiftLogicalAssign,

            // ? :
            TokenKind::QuestionMark => Op::QuestionMark,
            TokenKind::Colon => Op::Colon,

            TokenKind::Plus => Op::Plus,
            TokenKind::Minus => Op::Minus,
            TokenKind::Multiply => Op::Multiply,
            TokenKind::Divide => Op::Divide,
            TokenKind::Modulus => Op::Mod,

            TokenKind::Or => Op::Or,
            TokenKind::And => Op::And,
            TokenKind::BitOr => Op::BitOr,
            TokenKind::BitXOr => Op::BitXOr,
            TokenKind::BitAnd => Op::BitAnd,

            TokenKind::Equals => Op::Equals,
            TokenKind::NotEquals => Op::NotEquals,
            TokenKind::IdentityEquals => Op::IdentityEquals,
            TokenKind::IdentityNotEquals => Op::IdentityNotEquals,

            TokenKind::LessThan => Op::LessThan,
            TokenKind::LessThanEquals => Op::LessThanEquals,
            TokenKind::MoreThan => Op::MoreThan,
            TokenKind::GreaterThanEquals => Op::GreaterThanEquals,
            TokenKind::KeyWord(KeyWordKind::In) => Op::In,
            TokenKind::KeyWord(KeyWordKind::Instanceof) => Op::Instanceof,
            TokenKind::KeyWord(KeyWordKind::As) => Op::As,

            TokenKind::LeftShiftArithmetic => Op::LeftShiftArithmetic,
            TokenKind::RightShiftArithmetic => Op::RightShiftArithmetic,
            TokenKind::RightShiftLogical => Op::RightShiftLogical,

            _ => unreachable!(),
        };

        self.forward();
        Ok(op)
    }

    fn extract_exp_from_stack(
        &mut self,
        mut op_stack: Vec<Op>,
        mut exp_stack: Vec<ASTNode<Exp>>,
    ) -> ParseResult<ASTNode<Exp>> {
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
    ) -> ParseResult<()> {
        loop {
            if op_stack.is_empty() {
                op_stack.push(op);
                break;
            }

            // 如果优先级无法爬山
            if let Some(true) = op_stack.last().map(|top_op| op.hold(top_op)) {
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
    ) -> ParseResult<()> {
        let op = op_stack.last().unwrap();

        if op.is_bin_op() {
            let op = op_stack.pop().unwrap();
            if let (Some(right), Some(left)) = (exp_stack.pop(), exp_stack.pop()) {
                let begin = left.info.span.get_begin();
                let end = right.info.span.get_end();
                let exp = Exp::BinaryExp(BinaryExp::new(left, op, right));
                exp_stack.push(ASTNode::new(exp, Span::new(begin, end)));
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

            let begin = cond.info.span.get_begin();
            let end = false_br.info.span.get_end();

            let exp = Exp::TernaryExp(TernaryExp::new(cond, true_br, false_br));
            exp_stack.push(ASTNode::new(exp, Span::new(begin, end)));
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
            | TokenKind::Modulus

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
