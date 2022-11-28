use crate::{
    error::TSError, ir::IR, lexer::Lexer, parser::Parser, sematics::SematicsWalker,
    utils::get_char_stream,
};

pub struct Compiler {
    filename: String,
}

impl Compiler {
    pub fn new(filename: &str) -> Self {
        Compiler {
            filename: filename.to_owned(),
        }
    }

    pub fn run(&self) -> Result<(), TSError> {
        let char_stream = get_char_stream(&self.filename);
        let mut lexer = Lexer::new(&char_stream, &self.filename);
        let token_stream = lexer.get_token_stream()?;
        let mut parser = Parser::new(token_stream, &self.filename);
        let mut ast = parser.parse()?;
        ast.vis(&format!("{}.dot", self.filename));
        let env = SematicsWalker::walk(ast.get_program_ref())?;
        // let ir = IR::gen_ir(ast);

        Ok(())
    }
}
