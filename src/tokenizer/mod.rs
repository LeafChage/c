mod parser;
mod token;

use combine::EasyParser;
pub use token::Token;

pub fn tokenize(src: &str) -> Vec<Token> {
    parser::tokens().easy_parse(src).expect("TODO").0
}
