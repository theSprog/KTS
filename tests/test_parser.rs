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
    fn test_import() {
        test_parser("resource/parser/import/01.ts");
    }

    #[test]
    fn test_export() {
        test_parser("resource/parser/export/01.ts");
    }

    #[test]
    fn test_class() {
        test_parser("resource/parser/class/01.ts");
        test_parser("resource/parser/class/02.ts");
        test_parser("resource/parser/class/03.ts");
        test_parser("resource/parser/class/04.ts");
    }

    #[test]
    fn test_interface() {
        test_parser("resource/parser/interface/01.ts");
        test_parser("resource/parser/interface/02.ts");
    }

    #[test]
    fn test_temp() {
        test_parser("resource/parser/temp/temp.ts");
    }
}
