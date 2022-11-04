use crate::ast::{visulize::Visualizable, AST_GRAPH};

#[derive(Debug, Default)]
pub enum Value {
    Number(f64),
    Integer(i32),
    String(String),
    Boolean(bool),

    #[default]
    Null,
}

impl Visualizable for Value {
    fn draw(&self, id: usize) {
        match self {
            Value::Number(number) => todo!(),
            Value::Integer(integer) => todo!(),
            Value::String(string) => {
                let string = format!("\\\"{}\\\"", string);
                AST_GRAPH::put_node(id, &string);
            }
            Value::Boolean(boolean) => todo!(),
            Value::Null => todo!(),
        }
    }
}

#[derive(Default)]
pub struct Identifier {
    context: String,
}

impl Identifier {
    pub fn new(context: &str) -> Self {
        Identifier {
            context: context.to_string(),
        }
    }
}

impl Visualizable for Identifier {
    fn draw(&self, id: usize) {
        AST_GRAPH::put_node(id, &self.context);
    }
}
