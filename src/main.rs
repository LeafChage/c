#[macro_use]
extern crate combine;

mod parser;
mod tokenizer;
mod codegen;

// pub use parser::Node;
// use std::io::{Error, ErrorKind, Result};

// mod tokenize;
// pub use tokenize::Token;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // if let Some(src) = args.get(1) {
    //     let mut node = parser::parse(src);
    //     codegen::gen(node.pop().unwrap()).expect("OK");
    // }
}
