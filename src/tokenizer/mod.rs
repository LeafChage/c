mod parser;
mod token;

pub use token::Token;

pub fn tokenize(src: &str) -> Vec<Token> {
    parser::tokens(src.as_bytes()).expect("TODO").0
}
