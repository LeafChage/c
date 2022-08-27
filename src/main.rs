#[macro_use]
extern crate combine;
extern crate once_cell;

mod codegen;
mod parser;
mod tokenizer;

// pub use parser::Node;

// mod tokenize;
// pub use tokenize::Token;
use std::fs;
use std::io::Result;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if let Some(path) = args.get(1) {
        let src = fs::read_to_string(path)?;
        let tokenize = tokenizer::tokenize(&src);
        let node = parser::parse(&tokenize[..]).unwrap();
        codegen::codegen(node);
    }
    Ok(())
}
