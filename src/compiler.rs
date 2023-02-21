use std::sync::Mutex;

use crate::{
    ast::AST,
    error::TSError,
    eval::Eval,
    ir::{IRKind, IR},
    lexer::Lexer,
    parser::Parser,
    sematics::SematicsWalker,
    utils::get_char_stream,
};

pub struct Compiler {
    pub(crate) filename: String,
    show_ast: bool,
}

impl Compiler {
    pub fn new(filename: &str) -> Self {
        Compiler {
            filename: filename.to_owned(),
            show_ast: false,
        }
    }

    pub fn set_show_ast(mut self) -> Self {
        self.show_ast = true;
        self
    }

    pub fn run(&self) -> Result<(), TSError> {
        let ast = self.gen_ast()?;
        if self.show_ast {
            self.visualize(&ast);
        }
        // self.eval(&ast)?;

        // let env = SematicsWalker::walk(ast.get_program_ref())?;
        // let ir = IR::gen_ir(ast, IRKind::LLVM);

        Ok(())
    }

    // front part
    fn gen_ast(&self) -> Result<AST, TSError> {
        let char_stream = get_char_stream(&self.filename);
        let mut lexer = Lexer::new(&char_stream, &self.filename);
        let token_stream = lexer.get_token_stream()?;
        let mut parser = Parser::new(token_stream, &self.filename);
        parser.parse()
    }

    // plugin part
    fn visualize(&self, ast: &AST) {
        ast.vis(&format!("{}.dot", self.filename.replace(".ts", "")));
    }

    fn eval(&self, ast: &AST) -> Result<(), TSError> {
        Ok(println!("{:?}", Eval::walk(ast)?))
    }
}
