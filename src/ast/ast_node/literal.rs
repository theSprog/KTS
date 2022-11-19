use crate::ast::{visulize::Visualizable, AstGraph, NodeInfo};

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
    fn draw(&self, self_info: NodeInfo, graph: &mut AstGraph) {
        match self {
            Literal::Number(number) => {
                graph.put_node(self_info, &number.to_string());
            }
            Literal::Integer(integer) => {
                graph.put_node(self_info, &integer.to_string());
            }
            Literal::String(string) => {
                let string = format!("\\\"{}\\\"", string);
                graph.put_node(self_info, &string);
            }
            Literal::Boolean(boolean) => {
                let boolean_str = if *boolean { "true" } else { "false" };
                graph.put_node(self_info, boolean_str);
            }
            Literal::Null => {
                graph.put_node(self_info, "null");
            }
        }
    }
}
