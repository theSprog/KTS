use self::{env::Env, symbol::Symbol};

pub mod env;
pub mod symbol;

#[test]
fn test() {
    let mut global = Env::new_global();
    global.put_symbol("abc", symbol::Symbol::Class);
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
