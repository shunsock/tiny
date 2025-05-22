mod ast;
mod data_type;
mod parser;
mod tokenizer;
mod compiler;
mod opcode;

use ast::Stmt;
use compiler::{Compiler, compile_error_to_message};
use opcode::OpCode;
use parser::{Parser, parse_error_to_message};
use std::env;
use std::process::exit;
use tokenizer::{Tokenizer, tokenize_error_to_message};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", format_args!("{:?}", args));

    let tokens = match Tokenizer::tokenize(args[1].as_str()) {
        Ok(tokens) => tokens,
        Err(e) => {
            println!("{}", tokenize_error_to_message(e));
            exit(1)
        }
    };
    println!("{:?}", tokens);

    let ast: Stmt = Parser::new(tokens).parse().unwrap_or_else(|e| {
        println!("{}", parse_error_to_message(e));
        exit(1)
    });
    println!("{:?}", ast.clone());

    let mut compiler = Compiler::new();
    let opcodes: Vec<OpCode> = compiler.compile_stmt(&ast).unwrap_or_else(|e|{
        println!("{}", compile_error_to_message(e));
        exit(1)
    });
    println!("{:?}", opcodes);
}
