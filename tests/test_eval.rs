#[allow(dead_code, unused_imports)]
use kts::compiler::Compiler;
use kts::error::err_exit;

fn test_eval(filename: &str) {
    let compiler = Compiler::new(filename);
    let res = compiler.run();
    if let Err(err) = res {
        err_exit(err);
    }
}

#[test]
fn test_expr() {
    test_eval("resource/eval/expr/01.ts");
}
