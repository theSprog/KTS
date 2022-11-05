#![allow(warnings, unused)]

extern crate kts;

#[cfg(test)]
mod test_lexer {
    use kts::compiler::Compiler;
    use kts::error::err_exit;

    fn test_lexer(filename: &str) {
        let compiler = Compiler::new(filename);
        let res = compiler.run();
        match res {
            Ok(_) => {}
            Err(e) => err_exit(e),
        }
    }

    #[test]
    fn test01() {
        test_lexer("resource/lexer/test01");
    }
}
