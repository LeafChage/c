mod node;
mod parser;
use super::tokenizer::Token;

pub use node::Node;

pub fn parse(src: &[Token]) -> Vec<Node> {
    let mut p = parser::CParser::new();
    p.program(src).expect("TODO").0
}
