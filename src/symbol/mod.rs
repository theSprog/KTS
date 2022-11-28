pub mod env;

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

#[cfg(test)]
mod tests {
    use crate::symbol::{self, env::Env, Symbol};

    #[test]
    fn test() {
        let mut global = Env::new_global();
        global.put_symbol("abc", Symbol::Class);
        let mut sub1 = Env::new(&global);
        sub1.put_symbol("bcd", Symbol::Class);
        let abc = sub1.get_symbol("abc");
        assert_eq!(abc, Some(&Symbol::Class));
        let mut sub2 = Env::new(&global);
        sub2.put_symbol("def", Symbol::Class);

        let mut sub3 = Env::new(&mut sub2);
        sub3.put_symbol("bcd", Symbol::Namespace);
        let s = sub3.get_symbol("bcd");
        // assert!(*s.unwrap() == Symbol::Namespace);
        assert_eq!(s, Some(&Symbol::Namespace))
    }
}
