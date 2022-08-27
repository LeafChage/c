mod node;
mod parser;
use super::tokenizer::Token;

pub use node::Node;
pub use parser::{Error, Result};

pub fn parse(src: &[Token]) -> Result<Vec<Node>> {
    let mut p = parser::Parser::new();
    p.program(src).map(|(nodes, _)| nodes)
}
