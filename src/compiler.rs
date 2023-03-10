use lazy_static::lazy_static;
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

lazy_static! {
    pub static ref FILENAME: Mutex<String> = Mutex::new(String::new());
}

pub type IResult<T> = Result<T, TSError>;

pub struct Compiler {
    pub(crate) filename: String,
    show_ast: bool,
}

impl Compiler {
    pub fn new(filename: &str) -> Self {
        FILENAME.lock().unwrap().clear();
        FILENAME.lock().unwrap().push_str(filename);

        Compiler {
            filename: filename.to_owned(),
            show_ast: false,
        }
    }

    pub(crate) fn filename() -> String {
        FILENAME.lock().unwrap().clone()
    }

    pub fn set_show_ast(mut self) -> Self {
        self.show_ast = true;
        self
    }

    pub fn run(&self) -> IResult<()> {
        let ast = self.gen_ast()?;
        if self.show_ast {
            self.visualize(&ast);
        } else {
            // because eval is not finished
            self.eval(&ast)?;
        }

        // let env = SematicsWalker::walk(ast.get_program_ref())?;
        // let ir = IR::gen_ir(ast, IRKind::LLVM);

        Ok(())
    }

    // front part
    fn gen_ast(&self) -> IResult<AST> {
        let char_stream = get_char_stream(&self.filename);
        let mut lexer = Lexer::new(&char_stream);
        let token_stream = lexer.get_token_stream()?;
        let mut parser = Parser::new(token_stream);
        parser.parse()
    }

    // plugin part
    fn visualize(&self, ast: &AST) {
        ast.vis(&format!("{}.dot", self.filename.replace(".ts", "")));
    }

    fn eval(&self, ast: &AST) -> IResult<()> {
        Ok(println!("{:?}", Eval::walk(ast)?))
    }
}
