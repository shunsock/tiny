mod ast;
mod data_type;
mod tokenizer;

use std::env;
use tokenizer::{
    Tokenizer,
    tokenize_error_to_message
};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", format_args!("{:?}", args));
    match Tokenizer::tokenize(args[1].as_str()) {
        Ok(tokens) => println!("{:?}", tokens),
        Err(e) => println!("{}", tokenize_error_to_message(e)),
    }
}
