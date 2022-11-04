use crate::{error::TSError, lexer::Lexer, parser::Parser, utils::get_char_stream};

pub struct Compiler<'a> {
    filename: &'a str,
}

impl<'a> Compiler<'a> {
    pub fn new(filename: &'a str) -> Self {
        Compiler { filename }
    }

    pub fn run(&self) -> Result<(), TSError> {
        let char_stream = get_char_stream(self.filename);
        let mut lexer = Lexer::new(&char_stream);
        let mut parser = Parser::new(lexer.get_token_stream()?);
        let mut ast = parser.parse()?;
        ast.vis(&format!("{}.dot", self.filename));

        // let mut parser = Parser::new(tokens);
        // parser.parse();
        Ok(())
    }
}
