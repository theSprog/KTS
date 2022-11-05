use std::fmt::{Display, Formatter};

use crate::ast::{AST_GRAPH};
use crate::ast::visulize::Visualizable;

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
    Function_,   // function
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
    fn draw(&self, id: usize) {
        match self {
            KeyWordKind::Let => AST_GRAPH::put_node(id, "let"),
            KeyWordKind::Any => AST_GRAPH::put_node(id, "any"),
            KeyWordKind::Number => AST_GRAPH::put_node(id, "number"),
            KeyWordKind::Boolean => AST_GRAPH::put_node(id, "boolean"),
            KeyWordKind::String => AST_GRAPH::put_node(id, "string"),
            KeyWordKind::Symbol => AST_GRAPH::put_node(id, "symbol"),
            KeyWordKind::True => AST_GRAPH::put_node(id, "true"),
            KeyWordKind::False => AST_GRAPH::put_node(id, "false"),
            KeyWordKind::Null => AST_GRAPH::put_node(id, "null"),
            KeyWordKind::TypeAlias => AST_GRAPH::put_node(id, "typealias"),
            KeyWordKind::Get => AST_GRAPH::put_node(id, "get"),
            KeyWordKind::Set => AST_GRAPH::put_node(id, "set"),
            KeyWordKind::Constructor => AST_GRAPH::put_node(id, "constructor"),
            KeyWordKind::Namespace => AST_GRAPH::put_node(id, "namespace"),
            KeyWordKind::Require => AST_GRAPH::put_node(id, "require"),
            KeyWordKind::Module => AST_GRAPH::put_node(id, "module"),
            KeyWordKind::Declare => AST_GRAPH::put_node(id, "declare"),
            KeyWordKind::Abstract => AST_GRAPH::put_node(id, "abstract"),
            KeyWordKind::Is => AST_GRAPH::put_node(id, "is"),
            KeyWordKind::Implements => AST_GRAPH::put_node(id, "implements"),
            KeyWordKind::Private => AST_GRAPH::put_node(id, "private"),
            KeyWordKind::Public => AST_GRAPH::put_node(id, "public"),
            KeyWordKind::Interface => AST_GRAPH::put_node(id, "interface"),
            KeyWordKind::Package => AST_GRAPH::put_node(id, "package"),
            KeyWordKind::Protected => AST_GRAPH::put_node(id, "protected"),
            KeyWordKind::Static => AST_GRAPH::put_node(id, "static"),
            KeyWordKind::Yield => AST_GRAPH::put_node(id, "yield"),
            KeyWordKind::Break => AST_GRAPH::put_node(id, "break"),
            KeyWordKind::Do => AST_GRAPH::put_node(id, "do"),
            KeyWordKind::Instanceof => AST_GRAPH::put_node(id, "instanceof"),
            KeyWordKind::Typeof => AST_GRAPH::put_node(id, "typeof"),
            KeyWordKind::Case => AST_GRAPH::put_node(id, "case"),
            KeyWordKind::Else => AST_GRAPH::put_node(id, "else"),
            KeyWordKind::New => AST_GRAPH::put_node(id, "new"),
            KeyWordKind::Var => AST_GRAPH::put_node(id, "var"),
            KeyWordKind::Catch => AST_GRAPH::put_node(id, "catch"),
            KeyWordKind::Finally => AST_GRAPH::put_node(id, "finally"),
            KeyWordKind::Return => AST_GRAPH::put_node(id, "return"),
            KeyWordKind::Void => AST_GRAPH::put_node(id, "void"),
            KeyWordKind::Continue => AST_GRAPH::put_node(id, "continue"),
            KeyWordKind::For => AST_GRAPH::put_node(id, "for"),
            KeyWordKind::Switch => AST_GRAPH::put_node(id, "switch"),
            KeyWordKind::While => AST_GRAPH::put_node(id, "while"),
            KeyWordKind::Debugger => AST_GRAPH::put_node(id, "debugger"),
            KeyWordKind::Function_ => AST_GRAPH::put_node(id, "function_"),
            KeyWordKind::This => AST_GRAPH::put_node(id, "this"),
            KeyWordKind::With => AST_GRAPH::put_node(id, "with"),
            KeyWordKind::Default => AST_GRAPH::put_node(id, "default"),
            KeyWordKind::If => AST_GRAPH::put_node(id, "if"),
            KeyWordKind::Throw => AST_GRAPH::put_node(id, "throw"),
            KeyWordKind::Delete => AST_GRAPH::put_node(id, "delete"),
            KeyWordKind::In => AST_GRAPH::put_node(id, "in"),
            KeyWordKind::Try => AST_GRAPH::put_node(id, "try"),
            KeyWordKind::As => AST_GRAPH::put_node(id, "as"),
            KeyWordKind::From => AST_GRAPH::put_node(id, "from"),
            KeyWordKind::ReadOnly => AST_GRAPH::put_node(id, "readonly"),
            KeyWordKind::Async => AST_GRAPH::put_node(id, "async"),
            KeyWordKind::Class => AST_GRAPH::put_node(id, "class"),
            KeyWordKind::Enum => AST_GRAPH::put_node(id, "enum"),
            KeyWordKind::Extends => AST_GRAPH::put_node(id, "extends"),
            KeyWordKind::Super => AST_GRAPH::put_node(id, "super"),
            KeyWordKind::Const => AST_GRAPH::put_node(id, "const"),
            KeyWordKind::Export => AST_GRAPH::put_node(id, "export"),
            KeyWordKind::Import => AST_GRAPH::put_node(id, "import"),
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
            KeyWordKind::Function_ => token_string!(f, "keyworld-function"),
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
    Equals_,        // ==
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
    fn draw(&self, id: usize) {
        match self {
            // 这些 token 不可能到达,我们需要单独处理他们
            TokenKind::Identifier
            | TokenKind::KeyWord(_)
            | TokenKind::Number
            | TokenKind::String
            | TokenKind::EOF => unreachable!(),

            TokenKind::At => AST_GRAPH::put_node(id, "@"),
            TokenKind::LeftParen => AST_GRAPH::put_node(id, "("),
            TokenKind::RightParen => AST_GRAPH::put_node(id, ")"),
            TokenKind::LeftBracket => AST_GRAPH::put_node(id, "{"),
            TokenKind::RightBracket => AST_GRAPH::put_node(id, "}"),
            TokenKind::LeftBrace => AST_GRAPH::put_node(id, "["),
            TokenKind::RightBrace => AST_GRAPH::put_node(id, "]"),
            TokenKind::Comma => AST_GRAPH::put_node(id, ","),
            TokenKind::SemiColon => AST_GRAPH::put_node(id, ";"),
            TokenKind::QuestionMark => AST_GRAPH::put_node(id, "?"),
            TokenKind::Colon => AST_GRAPH::put_node(id, ":"),
            TokenKind::BitNot => AST_GRAPH::put_node(id, "~"),
            TokenKind::IdentityNotEquals => AST_GRAPH::put_node(id, "!=="),
            TokenKind::NotEquals => AST_GRAPH::put_node(id, "!="),
            TokenKind::Not => AST_GRAPH::put_node(id, "!"),
            TokenKind::MultiplyAssign => AST_GRAPH::put_node(id, "*="),
            TokenKind::Multiply => AST_GRAPH::put_node(id, "*"),
            TokenKind::DivideAssign => AST_GRAPH::put_node(id, "/="),
            TokenKind::Divide => AST_GRAPH::put_node(id, "/"),
            TokenKind::ModulusAssign => AST_GRAPH::put_node(id, "%="),
            TokenKind::Modulus => AST_GRAPH::put_node(id, "%"),
            TokenKind::Ellipsis => AST_GRAPH::put_node(id, "..."),
            TokenKind::Dot => AST_GRAPH::put_node(id, "."),
            TokenKind::PlusPlus => AST_GRAPH::put_node(id, "++"),
            TokenKind::PlusAssign => AST_GRAPH::put_node(id, "+="),
            TokenKind::Plus => AST_GRAPH::put_node(id, "+"),
            TokenKind::MinusMinus => AST_GRAPH::put_node(id, "--"),
            TokenKind::MinusAssign => AST_GRAPH::put_node(id, "-="),
            TokenKind::Minus => AST_GRAPH::put_node(id, "-"),
            TokenKind::RightShiftLogicalAssign => AST_GRAPH::put_node(id, ">>>="),
            TokenKind::RightShiftArithmeticAssign => AST_GRAPH::put_node(id, ">>="),
            TokenKind::RightShiftLogical => AST_GRAPH::put_node(id, ">>>"),
            TokenKind::RightShiftArithmetic => AST_GRAPH::put_node(id, ">>"),
            TokenKind::GreaterThanEquals => AST_GRAPH::put_node(id, ">="),
            TokenKind::MoreThan => AST_GRAPH::put_node(id, ">"),
            TokenKind::LeftShiftArithmeticAssign => AST_GRAPH::put_node(id, "<<="),
            TokenKind::LeftShiftArithmetic => AST_GRAPH::put_node(id, "<<"),
            TokenKind::LessThanEquals => AST_GRAPH::put_node(id, "<="),
            TokenKind::LessThan => AST_GRAPH::put_node(id, "<"),
            TokenKind::IdentityEquals => AST_GRAPH::put_node(id, "==="),
            TokenKind::Equals_ => AST_GRAPH::put_node(id, "=="),
            TokenKind::ARROW => AST_GRAPH::put_node(id, "=>"),
            TokenKind::Assign => AST_GRAPH::put_node(id, "="),
            TokenKind::And => AST_GRAPH::put_node(id, "&&"),
            TokenKind::BitAndAssign => AST_GRAPH::put_node(id, "&="),
            TokenKind::BitAnd => AST_GRAPH::put_node(id, "&"),
            TokenKind::BitXorAssign => AST_GRAPH::put_node(id, "^="),
            TokenKind::BitXOr => AST_GRAPH::put_node(id, "^"),
            TokenKind::BitOrAssign => AST_GRAPH::put_node(id, "|="),
            TokenKind::Or => AST_GRAPH::put_node(id, "||"),
            TokenKind::BitOr => AST_GRAPH::put_node(id, "|"),
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
            TokenKind::Equals_ => token_string!(f, "equals"),
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
