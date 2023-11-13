mod tokenizer;

use std::{env, fs};
use std::process::exit;

use tokenizer::{Tokenizer, TokenType, Token};

fn main() {
    let args: Vec<String> = env::args().collect();

    // first argument is path of the compiler
    if args.len() != 2 {
        println!("Please specify a source file");
        println!("E.g.: carbon <test.ca>");
        exit(1)
    }

    let filepath = &args[1];
    let data = fs::read_to_string(filepath).expect("Unable to read file");
    let mut tokenizer = Tokenizer::new(data.as_str());
    let tokens = tokenizer.tokenize();

    println!("{:?}", tokens);
}