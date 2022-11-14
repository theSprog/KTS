use std::fmt::{Display, Formatter};

use crate::ast::visulize::Visualizable;
use crate::ast::AstGraph;

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
    fn draw(&self, id: usize, graph: &mut AstGraph) {
        match self {
            KeyWordKind::Let => graph.put_node(id, "let"),
            KeyWordKind::Any => graph.put_node(id, "any"),
            KeyWordKind::Number => graph.put_node(id, "number"),
            KeyWordKind::Boolean => graph.put_node(id, "boolean"),
            KeyWordKind::String => graph.put_node(id, "string"),
            KeyWordKind::Symbol => graph.put_node(id, "symbol"),
            KeyWordKind::True => graph.put_node(id, "true"),
            KeyWordKind::False => graph.put_node(id, "false"),
            KeyWordKind::Null => graph.put_node(id, "null"),
            KeyWordKind::TypeAlias => graph.put_node(id, "typealias"),
            KeyWordKind::Get => graph.put_node(id, "get"),
            KeyWordKind::Set => graph.put_node(id, "set"),
            KeyWordKind::Constructor => graph.put_node(id, "constructor"),
            KeyWordKind::Namespace => graph.put_node(id, "namespace"),
            KeyWordKind::Require => graph.put_node(id, "require"),
            KeyWordKind::Module => graph.put_node(id, "module"),
            KeyWordKind::Declare => graph.put_node(id, "declare"),
            KeyWordKind::Abstract => graph.put_node(id, "abstract"),
            KeyWordKind::Is => graph.put_node(id, "is"),
            KeyWordKind::Implements => graph.put_node(id, "implements"),
            KeyWordKind::Private => graph.put_node(id, "private"),
            KeyWordKind::Public => graph.put_node(id, "public"),
            KeyWordKind::Interface => graph.put_node(id, "interface"),
            KeyWordKind::Package => graph.put_node(id, "package"),
            KeyWordKind::Protected => graph.put_node(id, "protected"),
            KeyWordKind::Static => graph.put_node(id, "static"),
            KeyWordKind::Yield => graph.put_node(id, "yield"),
            KeyWordKind::Break => graph.put_node(id, "break"),
            KeyWordKind::Do => graph.put_node(id, "do"),
            KeyWordKind::Instanceof => graph.put_node(id, "instanceof"),
            KeyWordKind::Typeof => graph.put_node(id, "typeof"),
            KeyWordKind::Case => graph.put_node(id, "case"),
            KeyWordKind::Else => graph.put_node(id, "else"),
            KeyWordKind::New => graph.put_node(id, "new"),
            KeyWordKind::Var => graph.put_node(id, "var"),
            KeyWordKind::Catch => graph.put_node(id, "catch"),
            KeyWordKind::Finally => graph.put_node(id, "finally"),
            KeyWordKind::Return => graph.put_node(id, "return"),
            KeyWordKind::Void => graph.put_node(id, "void"),
            KeyWordKind::Continue => graph.put_node(id, "continue"),
            KeyWordKind::For => graph.put_node(id, "for"),
            KeyWordKind::Switch => graph.put_node(id, "switch"),
            KeyWordKind::While => graph.put_node(id, "while"),
            KeyWordKind::Debugger => graph.put_node(id, "debugger"),
            KeyWordKind::Function => graph.put_node(id, "function"),
            KeyWordKind::This => graph.put_node(id, "this"),
            KeyWordKind::With => graph.put_node(id, "with"),
            KeyWordKind::Default => graph.put_node(id, "default"),
            KeyWordKind::If => graph.put_node(id, "if"),
            KeyWordKind::Throw => graph.put_node(id, "throw"),
            KeyWordKind::Delete => graph.put_node(id, "delete"),
            KeyWordKind::In => graph.put_node(id, "in"),
            KeyWordKind::Try => graph.put_node(id, "try"),
            KeyWordKind::As => graph.put_node(id, "as"),
            KeyWordKind::From => graph.put_node(id, "from"),
            KeyWordKind::ReadOnly => graph.put_node(id, "readonly"),
            KeyWordKind::Async => graph.put_node(id, "async"),
            KeyWordKind::Class => graph.put_node(id, "class"),
            KeyWordKind::Enum => graph.put_node(id, "enum"),
            KeyWordKind::Extends => graph.put_node(id, "extends"),
            KeyWordKind::Super => graph.put_node(id, "super"),
            KeyWordKind::Const => graph.put_node(id, "const"),
            KeyWordKind::Export => graph.put_node(id, "export"),
            KeyWordKind::Import => graph.put_node(id, "import"),
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
    fn draw(&self, id: usize, graph: &mut AstGraph) {
        match self {
            // 这些 token 不可能到达,我们需要单独处理他们
            TokenKind::Identifier
            | TokenKind::KeyWord(_)
            | TokenKind::Number
            | TokenKind::String
            | TokenKind::EOF => unreachable!(),

            TokenKind::At => graph.put_node(id, "@"),
            TokenKind::LeftParen => graph.put_node(id, "("),
            TokenKind::RightParen => graph.put_node(id, ")"),
            TokenKind::LeftBracket => graph.put_node(id, "{"),
            TokenKind::RightBracket => graph.put_node(id, "}"),
            TokenKind::LeftBrace => graph.put_node(id, "["),
            TokenKind::RightBrace => graph.put_node(id, "]"),
            TokenKind::Comma => graph.put_node(id, ","),
            TokenKind::SemiColon => graph.put_node(id, ";"),
            TokenKind::QuestionMark => graph.put_node(id, "?"),
            TokenKind::Colon => graph.put_node(id, ":"),
            TokenKind::BitNot => graph.put_node(id, "~"),
            TokenKind::IdentityNotEquals => graph.put_node(id, "!=="),
            TokenKind::NotEquals => graph.put_node(id, "!="),
            TokenKind::Not => graph.put_node(id, "!"),
            TokenKind::MultiplyAssign => graph.put_node(id, "*="),
            TokenKind::Multiply => graph.put_node(id, "*"),
            TokenKind::DivideAssign => graph.put_node(id, "/="),
            TokenKind::Divide => graph.put_node(id, "/"),
            TokenKind::ModulusAssign => graph.put_node(id, "%="),
            TokenKind::Modulus => graph.put_node(id, "%"),
            TokenKind::Ellipsis => graph.put_node(id, "..."),
            TokenKind::Dot => graph.put_node(id, "."),
            TokenKind::PlusPlus => graph.put_node(id, "++"),
            TokenKind::PlusAssign => graph.put_node(id, "+="),
            TokenKind::Plus => graph.put_node(id, "+"),
            TokenKind::MinusMinus => graph.put_node(id, "--"),
            TokenKind::MinusAssign => graph.put_node(id, "-="),
            TokenKind::Minus => graph.put_node(id, "-"),
            TokenKind::RightShiftLogicalAssign => graph.put_node(id, ">>>="),
            TokenKind::RightShiftArithmeticAssign => graph.put_node(id, ">>="),
            TokenKind::RightShiftLogical => graph.put_node(id, ">>>"),
            TokenKind::RightShiftArithmetic => graph.put_node(id, ">>"),
            TokenKind::GreaterThanEquals => graph.put_node(id, ">="),
            TokenKind::MoreThan => graph.put_node(id, ">"),
            TokenKind::LeftShiftArithmeticAssign => graph.put_node(id, "<<="),
            TokenKind::LeftShiftArithmetic => graph.put_node(id, "<<"),
            TokenKind::LessThanEquals => graph.put_node(id, "<="),
            TokenKind::LessThan => graph.put_node(id, "<"),
            TokenKind::IdentityEquals => graph.put_node(id, "==="),
            TokenKind::Equals => graph.put_node(id, "=="),
            TokenKind::ARROW => graph.put_node(id, "=>"),
            TokenKind::Assign => graph.put_node(id, "="),
            TokenKind::And => graph.put_node(id, "&&"),
            TokenKind::BitAndAssign => graph.put_node(id, "&="),
            TokenKind::BitAnd => graph.put_node(id, "&"),
            TokenKind::BitXorAssign => graph.put_node(id, "^="),
            TokenKind::BitXOr => graph.put_node(id, "^"),
            TokenKind::BitOrAssign => graph.put_node(id, "|="),
            TokenKind::Or => graph.put_node(id, "||"),
            TokenKind::BitOr => graph.put_node(id, "|"),
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
