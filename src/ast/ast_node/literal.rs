use crate::ast::{visulize::Visualizable, AstGraph};

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
    fn draw(&self, id: usize, graph: &mut AstGraph) {
        match self {
            Value::Number(number) => todo!(),
            Value::Integer(integer) => todo!(),
            Value::String(string) => {
                let string = format!("\\\"{}\\\"", string);
                graph.put_node(id, &string);
            }
            Value::Boolean(boolean) => todo!(),
            Value::Null => todo!(),
        }
    }
}

