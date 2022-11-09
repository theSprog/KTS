use crate::ast::{visulize::Visualizable, AstGraph};

#[derive(Debug, Default)]
pub enum Literal {
    Number(f64),
    Integer(i32),
    String(String),
    Boolean(bool),

    #[default]
    Null,
}

impl Visualizable for Literal {
    fn draw(&self, self_id: usize, graph: &mut AstGraph) {
        match self {
            Literal::Number(number) => {
                graph.put_node(self_id, &number.to_string());
            }
            Literal::Integer(integer) => {
                graph.put_node(self_id, &integer.to_string());
            }
            Literal::String(string) => {
                let string = format!("\\\"{}\\\"", string);
                graph.put_node(self_id, &string);
            }
            Literal::Boolean(boolean) => {
                let boolean_str = if *boolean { "true" } else { "false" };
                graph.put_node(self_id, boolean_str);
            }
            Literal::Null => {
                graph.put_node(self_id, "null");
            }
        }
    }
}
