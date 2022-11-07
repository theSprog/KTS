extern crate kts;

#[cfg(test)]
mod test_parser {
    #[allow(dead_code, unused_imports)]
    use kts::compiler::Compiler;
    use kts::error::err_exit;

    fn test_parser(filename: &str) {
        let compiler = Compiler::new(filename);
        let res = compiler.run();
        match res {
            Ok(_) => {}
            Err(e) => err_exit(e),
        }
    }

    #[test]
    fn test01() {
        test_parser("resource/parser/import/import.ts");
    }

    #[test]
    fn test02() {
        test_parser("resource/parser/export/export.ts");
    }

    #[test]
    fn test03() {
        test_parser("resource/parser/class/class01.ts");
        test_parser("resource/parser/class/class02.ts");
        test_parser("resource/parser/class/class03.ts");
    }

    #[test]
    fn test_temp() {
        test_parser("resource/parser/temp/temp.ts");
    }
}
