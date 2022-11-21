#[derive(Debug, PartialEq)]
pub(crate) enum Symbol {
    Var(VarSymbol),   // 例如普通的 a, b, c
    Func(FuncSymbol), // 函数符号
    Class,            // class 符号
    Namespace,        // namespace 符号
}

#[derive(Debug, PartialEq)]
pub(crate) struct VarSymbol {
    var_type: VarType,         // 变量类型, 无类型则为 any
    initializer: Option<Data>, // 初始值
}

#[derive(Debug, PartialEq)]
pub(crate) enum Data {
    String(String), // 字符串字面量
    Number(Number),
    Boolean(Boolean),
    // Func,    // 没有函数类型的变量, 他被计入 FuncSymbol 中
    Class,
    Void,
    Null,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Number {
    I32(i32),
    F64(f64),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Boolean {
    True,
    False,
}
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum VarType {
    Any,
    String,
    Number,
    Boolean,
    Void,
    Null,
}

#[derive(Debug, PartialEq)]
pub(crate) struct FuncSymbol {
    args: Vec<Symbol>, // 参数类型
}
