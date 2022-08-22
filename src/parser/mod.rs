mod node;
mod parser;
use super::tokenizer::Token;
use combine::EasyParser;

pub use node::{Node, Sign};

pub fn parse(src: &[Token]) -> Vec<Node> {
    parser::program().easy_parse(src).expect("TODO").0
}
