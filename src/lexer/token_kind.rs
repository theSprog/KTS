use std::fmt::{Display, Formatter};

use crate::ast::visulize::Visualizable;
use crate::ast::{AstGraph, NodeInfo};

macro_rules! token_string {
    // macth like arm for macro
    ($f:ident,$s:expr) => {
        // macro expand to this code
        {
            write!($f, "{:>20}", $s)
        }
    };
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KeyWordKind {
    Let,         // let
    Any,         // any
    Number,      // number
    Boolean,     // boolean
    String,      // string
    Symbol,      // symbol
    True,        // true
    False,       // false
    Null,        // null
    TypeAlias,   // type
    Get,         // get
    Set,         // set
    Constructor, // constructor
    Namespace,   // namespace
    Require,     // require
    Module,      // module
    Declare,     // declare
    Abstract,    // abstract
    Is,          // is
    Implements,  // implements
    Private,     // private
    Public,      // public
    Interface,   // interface
    Package,     // package
    Protected,   // protected
    Static,      // static
    Yield,       // yield
    Break,       // break
    Do,          // do
    Instanceof,  // instanceof
    Typeof,      // typeof
    Case,        // case
    Else,        // else
    New,         // new
    Var,         // var
    Catch,       // catch
    Finally,     // finally
    Return,      // return
    Void,        // void
    Continue,    // continue
    For,         // for
    Switch,      // switch
    While,       // while
    Debugger,    // debugger
    Function,    // function
    This,        // this
    With,        // with
    Default,     // default
    If,          // if
    Throw,       // throw
    Delete,      // delete
    In,          // in
    Try,         // try
    As,          // as
    From,        // from
    ReadOnly,    // readonly
    Async,       // async

    ///,    Future Reserved Words
    Class, // class
    Enum,    // enum
    Extends, // extends
    Super,   // super
    Const,   // const
    Export,  // export
    Import,  // import
}

impl Visualizable for KeyWordKind {
    fn draw(&self, info: NodeInfo, graph: &mut AstGraph) {
        match self {
            KeyWordKind::Let => graph.put_node(info, "let"),
            KeyWordKind::Any => graph.put_node(info, "any"),
            KeyWordKind::Number => graph.put_node(info, "number"),
            KeyWordKind::Boolean => graph.put_node(info, "boolean"),
            KeyWordKind::String => graph.put_node(info, "string"),
            KeyWordKind::Symbol => graph.put_node(info, "symbol"),
            KeyWordKind::True => graph.put_node(info, "true"),
            KeyWordKind::False => graph.put_node(info, "false"),
            KeyWordKind::Null => graph.put_node(info, "null"),
            KeyWordKind::TypeAlias => graph.put_node(info, "typealias"),
            KeyWordKind::Get => graph.put_node(info, "get"),
            KeyWordKind::Set => graph.put_node(info, "set"),
            KeyWordKind::Constructor => graph.put_node(info, "constructor"),
            KeyWordKind::Namespace => graph.put_node(info, "namespace"),
            KeyWordKind::Require => graph.put_node(info, "require"),
            KeyWordKind::Module => graph.put_node(info, "module"),
            KeyWordKind::Declare => graph.put_node(info, "declare"),
            KeyWordKind::Abstract => graph.put_node(info, "abstract"),
            KeyWordKind::Is => graph.put_node(info, "is"),
            KeyWordKind::Implements => graph.put_node(info, "implements"),
            KeyWordKind::Private => graph.put_node(info, "private"),
            KeyWordKind::Public => graph.put_node(info, "public"),
            KeyWordKind::Interface => graph.put_node(info, "interface"),
            KeyWordKind::Package => graph.put_node(info, "package"),
            KeyWordKind::Protected => graph.put_node(info, "protected"),
            KeyWordKind::Static => graph.put_node(info, "static"),
            KeyWordKind::Yield => graph.put_node(info, "yield"),
            KeyWordKind::Break => graph.put_node(info, "break"),
            KeyWordKind::Do => graph.put_node(info, "do"),
            KeyWordKind::Instanceof => graph.put_node(info, "instanceof"),
            KeyWordKind::Typeof => graph.put_node(info, "typeof"),
            KeyWordKind::Case => graph.put_node(info, "case"),
            KeyWordKind::Else => graph.put_node(info, "else"),
            KeyWordKind::New => graph.put_node(info, "new"),
            KeyWordKind::Var => graph.put_node(info, "var"),
            KeyWordKind::Catch => graph.put_node(info, "catch"),
            KeyWordKind::Finally => graph.put_node(info, "finally"),
            KeyWordKind::Return => graph.put_node(info, "return"),
            KeyWordKind::Void => graph.put_node(info, "void"),
            KeyWordKind::Continue => graph.put_node(info, "continue"),
            KeyWordKind::For => graph.put_node(info, "for"),
            KeyWordKind::Switch => graph.put_node(info, "switch"),
            KeyWordKind::While => graph.put_node(info, "while"),
            KeyWordKind::Debugger => graph.put_node(info, "debugger"),
            KeyWordKind::Function => graph.put_node(info, "function"),
            KeyWordKind::This => graph.put_node(info, "this"),
            KeyWordKind::With => graph.put_node(info, "with"),
            KeyWordKind::Default => graph.put_node(info, "default"),
            KeyWordKind::If => graph.put_node(info, "if"),
            KeyWordKind::Throw => graph.put_node(info, "throw"),
            KeyWordKind::Delete => graph.put_node(info, "delete"),
            KeyWordKind::In => graph.put_node(info, "in"),
            KeyWordKind::Try => graph.put_node(info, "try"),
            KeyWordKind::As => graph.put_node(info, "as"),
            KeyWordKind::From => graph.put_node(info, "from"),
            KeyWordKind::ReadOnly => graph.put_node(info, "readonly"),
            KeyWordKind::Async => graph.put_node(info, "async"),
            KeyWordKind::Class => graph.put_node(info, "class"),
            KeyWordKind::Enum => graph.put_node(info, "enum"),
            KeyWordKind::Extends => graph.put_node(info, "extends"),
            KeyWordKind::Super => graph.put_node(info, "super"),
            KeyWordKind::Const => graph.put_node(info, "const"),
            KeyWordKind::Export => graph.put_node(info, "export"),
            KeyWordKind::Import => graph.put_node(info, "import"),
        }
    }
}

impl Display for KeyWordKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyWordKind::Let => token_string!(f, "keyworld-let"),
            KeyWordKind::Any => token_string!(f, "keyworld-any"),
            KeyWordKind::Number => token_string!(f, "keyworld-number"),
            KeyWordKind::Boolean => token_string!(f, "keyworld-boolean"),
            KeyWordKind::String => token_string!(f, "keyworld-string"),
            KeyWordKind::Symbol => token_string!(f, "keyworld-symbol"),

            KeyWordKind::True => token_string!(f, "keyworld-true"),
            KeyWordKind::False => token_string!(f, "keyworld-false"),
            KeyWordKind::Null => token_string!(f, "keyworld-null"),

            KeyWordKind::TypeAlias => token_string!(f, "keyworld-typealias"),
            KeyWordKind::Get => token_string!(f, "keyworld-get"),
            KeyWordKind::Set => token_string!(f, "keyworld-set"),
            KeyWordKind::Constructor => token_string!(f, "keyworld-constructor"),
            KeyWordKind::Namespace => token_string!(f, "keyworld-namespace"),
            KeyWordKind::Require => token_string!(f, "keyworld-require"),
            KeyWordKind::Module => token_string!(f, "keyworld-module"),
            KeyWordKind::Declare => token_string!(f, "keyworld-declare"),
            KeyWordKind::Abstract => token_string!(f, "keyworld-abstract"),
            KeyWordKind::Is => token_string!(f, "keyworld-is"),
            KeyWordKind::Implements => token_string!(f, "keyworld-implements"),
            KeyWordKind::Private => token_string!(f, "keyworld-private"),
            KeyWordKind::Public => token_string!(f, "keyworld-public"),
            KeyWordKind::Interface => token_string!(f, "keyworld-interface"),
            KeyWordKind::Package => token_string!(f, "keyworld-package"),
            KeyWordKind::Protected => token_string!(f, "keyworld-protected"),
            KeyWordKind::Static => token_string!(f, "keyworld-static"),
            KeyWordKind::Yield => token_string!(f, "keyworld-yield"),
            KeyWordKind::Break => token_string!(f, "keyworld-break"),
            KeyWordKind::Do => token_string!(f, "keyworld-do"),
            KeyWordKind::Instanceof => token_string!(f, "keyworld-instanceof"),
            KeyWordKind::Typeof => token_string!(f, "keyworld-typeof"),
            KeyWordKind::Case => token_string!(f, "keyworld-case"),
            KeyWordKind::Else => token_string!(f, "keyworld-else"),
            KeyWordKind::New => token_string!(f, "keyworld-new"),
            KeyWordKind::Var => token_string!(f, "keyworld-var"),
            KeyWordKind::Catch => token_string!(f, "keyworld-catch"),
            KeyWordKind::Finally => token_string!(f, "keyworld-finally"),
            KeyWordKind::Return => token_string!(f, "keyworld-return"),
            KeyWordKind::Void => token_string!(f, "keyworld-void"),
            KeyWordKind::Continue => token_string!(f, "keyworld-continue"),
            KeyWordKind::For => token_string!(f, "keyworld-for"),
            KeyWordKind::Switch => token_string!(f, "keyworld-switch"),
            KeyWordKind::While => token_string!(f, "keyworld-while"),
            KeyWordKind::Debugger => token_string!(f, "keyworld-debugger"),
            KeyWordKind::Function => token_string!(f, "keyworld-function"),
            KeyWordKind::This => token_string!(f, "keyworld-this"),
            KeyWordKind::With => token_string!(f, "keyworld-with"),
            KeyWordKind::Default => token_string!(f, "keyworld-default"),
            KeyWordKind::If => token_string!(f, "keyworld-if"),
            KeyWordKind::Throw => token_string!(f, "keyworld-throw"),
            KeyWordKind::Delete => token_string!(f, "keyworld-delete"),
            KeyWordKind::In => token_string!(f, "keyworld-in"),
            KeyWordKind::Try => token_string!(f, "keyworld-try"),
            KeyWordKind::As => token_string!(f, "keyworld-as"),
            KeyWordKind::From => token_string!(f, "keyworld-from"),
            KeyWordKind::ReadOnly => token_string!(f, "keyworld-readonly"),
            KeyWordKind::Async => token_string!(f, "keyworld-async"),
            KeyWordKind::Class => token_string!(f, "keyworld-class"),
            KeyWordKind::Enum => token_string!(f, "keyworld-enum"),
            KeyWordKind::Extends => token_string!(f, "keyworld-extends"),
            KeyWordKind::Super => token_string!(f, "keyworld-super"),
            KeyWordKind::Const => token_string!(f, "keyworld-const"),
            KeyWordKind::Export => token_string!(f, "keyworld-export"),
            KeyWordKind::Import => token_string!(f, "keyworld-import"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    Identifier,
    KeyWord(KeyWordKind),
    Number,
    String,

    At, // @

    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // {
    RightBracket, // }
    LeftBrace,    // [
    RightBrace,   // ]
    Comma,        // ,
    SemiColon,    // ;
    QuestionMark, // ?
    Colon,        // :
    BitNot,       // ~

    IdentityNotEquals, // !==
    NotEquals,         // !=
    Not,               // !

    MultiplyAssign, // *=
    Multiply,       // *

    DivideAssign, // /=
    Divide,       // /

    ModulusAssign, // %=
    Modulus,       // %

    Ellipsis, // ...
    Dot,      // .

    PlusPlus,   // ++
    PlusAssign, // +=
    Plus,       // +

    MinusMinus,  // --
    MinusAssign, // -=
    Minus,       // -

    RightShiftLogicalAssign,    // >>>=
    RightShiftArithmeticAssign, // >>=
    RightShiftLogical,          // >>>
    RightShiftArithmetic,       // >>
    GreaterThanEquals,          // >=
    MoreThan,                   // >

    LeftShiftArithmeticAssign, // <<=
    LeftShiftArithmetic,       // <<
    LessThanEquals,            // <=
    LessThan,                  // <

    IdentityEquals, // ===
    Equals,         // ==
    ARROW,          // =>
    Assign,         // =

    And,          // &&
    BitAndAssign, // &=
    BitAnd,       // &

    BitXorAssign, // ^=
    BitXOr,       // ^

    BitOrAssign, // |=
    Or,          // ||
    BitOr,       // |

    EOF,
}

impl Visualizable for TokenKind {
    fn draw(&self, info: NodeInfo, graph: &mut AstGraph) {
        match self {
            // 这些 token 不可能到达,我们需要单独处理他们
            TokenKind::Identifier
            | TokenKind::KeyWord(_)
            | TokenKind::Number
            | TokenKind::String
            | TokenKind::EOF => unreachable!(),

            TokenKind::At => graph.put_node(info, "@"),
            TokenKind::LeftParen => graph.put_node(info, "("),
            TokenKind::RightParen => graph.put_node(info, ")"),
            TokenKind::LeftBracket => graph.put_node(info, "{"),
            TokenKind::RightBracket => graph.put_node(info, "}"),
            TokenKind::LeftBrace => graph.put_node(info, "["),
            TokenKind::RightBrace => graph.put_node(info, "]"),
            TokenKind::Comma => graph.put_node(info, ","),
            TokenKind::SemiColon => graph.put_node(info, ";"),
            TokenKind::QuestionMark => graph.put_node(info, "?"),
            TokenKind::Colon => graph.put_node(info, ":"),
            TokenKind::BitNot => graph.put_node(info, "~"),
            TokenKind::IdentityNotEquals => graph.put_node(info, "!=="),
            TokenKind::NotEquals => graph.put_node(info, "!="),
            TokenKind::Not => graph.put_node(info, "!"),
            TokenKind::MultiplyAssign => graph.put_node(info, "*="),
            TokenKind::Multiply => graph.put_node(info, "*"),
            TokenKind::DivideAssign => graph.put_node(info, "/="),
            TokenKind::Divide => graph.put_node(info, "/"),
            TokenKind::ModulusAssign => graph.put_node(info, "%="),
            TokenKind::Modulus => graph.put_node(info, "%"),
            TokenKind::Ellipsis => graph.put_node(info, "..."),
            TokenKind::Dot => graph.put_node(info, "."),
            TokenKind::PlusPlus => graph.put_node(info, "++"),
            TokenKind::PlusAssign => graph.put_node(info, "+="),
            TokenKind::Plus => graph.put_node(info, "+"),
            TokenKind::MinusMinus => graph.put_node(info, "--"),
            TokenKind::MinusAssign => graph.put_node(info, "-="),
            TokenKind::Minus => graph.put_node(info, "-"),
            TokenKind::RightShiftLogicalAssign => graph.put_node(info, ">>>="),
            TokenKind::RightShiftArithmeticAssign => graph.put_node(info, ">>="),
            TokenKind::RightShiftLogical => graph.put_node(info, ">>>"),
            TokenKind::RightShiftArithmetic => graph.put_node(info, ">>"),
            TokenKind::GreaterThanEquals => graph.put_node(info, ">="),
            TokenKind::MoreThan => graph.put_node(info, ">"),
            TokenKind::LeftShiftArithmeticAssign => graph.put_node(info, "<<="),
            TokenKind::LeftShiftArithmetic => graph.put_node(info, "<<"),
            TokenKind::LessThanEquals => graph.put_node(info, "<="),
            TokenKind::LessThan => graph.put_node(info, "<"),
            TokenKind::IdentityEquals => graph.put_node(info, "==="),
            TokenKind::Equals => graph.put_node(info, "=="),
            TokenKind::ARROW => graph.put_node(info, "=>"),
            TokenKind::Assign => graph.put_node(info, "="),
            TokenKind::And => graph.put_node(info, "&&"),
            TokenKind::BitAndAssign => graph.put_node(info, "&="),
            TokenKind::BitAnd => graph.put_node(info, "&"),
            TokenKind::BitXorAssign => graph.put_node(info, "^="),
            TokenKind::BitXOr => graph.put_node(info, "^"),
            TokenKind::BitOrAssign => graph.put_node(info, "|="),
            TokenKind::Or => graph.put_node(info, "||"),
            TokenKind::BitOr => graph.put_node(info, "|"),
        }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Identifier => token_string!(f, "identifier"),
            TokenKind::KeyWord(kind) => token_string!(f, kind),

            TokenKind::Number => token_string!(f, "number"),
            TokenKind::String => token_string!(f, "string"),

            TokenKind::At => token_string!(f, "at"),
            TokenKind::LeftParen => token_string!(f, "left paren"),
            TokenKind::RightParen => token_string!(f, "right paren"),
            TokenKind::LeftBracket => token_string!(f, "left bracket"),
            TokenKind::RightBracket => token_string!(f, "right bracket"),
            TokenKind::LeftBrace => token_string!(f, "left brace"),
            TokenKind::RightBrace => token_string!(f, "right brace"),

            TokenKind::Comma => token_string!(f, "comma"),
            TokenKind::SemiColon => token_string!(f, "semicolon"),
            TokenKind::QuestionMark => token_string!(f, "question mark"),
            TokenKind::Colon => token_string!(f, "colon"),
            TokenKind::BitNot => token_string!(f, "bit not"),

            TokenKind::IdentityNotEquals => token_string!(f, "identity not equals"),
            TokenKind::NotEquals => token_string!(f, "not equals"),
            TokenKind::Not => token_string!(f, "not"),

            TokenKind::MultiplyAssign => token_string!(f, "multiply assign"),
            TokenKind::Multiply => token_string!(f, "multiply"),

            TokenKind::DivideAssign => token_string!(f, "divide assign"),
            TokenKind::Divide => token_string!(f, "divide"),

            TokenKind::ModulusAssign => token_string!(f, "modulus assign"),
            TokenKind::Modulus => token_string!(f, "modulus"),

            TokenKind::Ellipsis => token_string!(f, "ellipsis"),
            TokenKind::Dot => token_string!(f, "dot"),

            TokenKind::PlusPlus => token_string!(f, "plus plus"),
            TokenKind::PlusAssign => token_string!(f, "plus assign"),
            TokenKind::Plus => token_string!(f, "plus"),

            TokenKind::MinusMinus => token_string!(f, "minus minus"),
            TokenKind::MinusAssign => token_string!(f, "minus assign"),
            TokenKind::Minus => token_string!(f, "minus"),

            TokenKind::RightShiftLogicalAssign => token_string!(f, "RS logical assign"),
            TokenKind::RightShiftArithmeticAssign => token_string!(f, "RS arith assign"),
            TokenKind::RightShiftLogical => token_string!(f, "RS logical"),
            TokenKind::RightShiftArithmetic => token_string!(f, "RS arith"),
            TokenKind::GreaterThanEquals => token_string!(f, "greaterThan equals"),
            TokenKind::MoreThan => token_string!(f, "more than"),

            TokenKind::LeftShiftArithmeticAssign => token_string!(f, "LS arith assign"),
            TokenKind::LeftShiftArithmetic => token_string!(f, "LS arith"),
            TokenKind::LessThanEquals => token_string!(f, "lessThan equals"),
            TokenKind::LessThan => token_string!(f, "lessThan"),

            TokenKind::IdentityEquals => token_string!(f, "identity equals"),
            TokenKind::Equals => token_string!(f, "equals"),
            TokenKind::ARROW => token_string!(f, "arrow"),
            TokenKind::Assign => token_string!(f, "assign"),

            TokenKind::And => token_string!(f, "and"),
            TokenKind::BitAndAssign => token_string!(f, "bit and assign"),
            TokenKind::BitAnd => token_string!(f, "bit and"),

            TokenKind::BitXorAssign => token_string!(f, "bit xor assign"),
            TokenKind::BitXOr => token_string!(f, "bit xor"),

            TokenKind::BitOrAssign => token_string!(f, "bit or assign"),
            TokenKind::Or => token_string!(f, "or"),
            TokenKind::BitOr => token_string!(f, "bit or"),

            TokenKind::EOF => token_string!(f, "EOF"),
        }
    }
}
