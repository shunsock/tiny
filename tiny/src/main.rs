mod ast;
mod compiler;
mod data_type;
mod opcode;
mod parser;
mod tokenizer;

use ast::Stmt;
use compiler::{Compiler, compile_error_to_message};
use opcode::OpCode;
use parser::{Parser, parse_error_to_message};
use std::env;
use std::process::exit;
use tokenizer::{Token, Tokenizer, tokenize_error_to_message};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", format_args!("{:?}", args));

    let tokens: Vec<Token> = Tokenizer::tokenize(args[1].as_str()).unwrap_or_else(|e| {
        println!("{}", tokenize_error_to_message(e));
        exit(1)
    });
    println!("{:?}", tokens);

    let ast: Stmt = Parser::new(tokens).parse().unwrap_or_else(|e| {
        println!("{}", parse_error_to_message(e));
        exit(1)
    });
    println!("{:?}", ast.clone());

    let mut compiler: Compiler = Compiler::new();
    let opcodes: Vec<OpCode> = compiler.compile_stmt(ast).unwrap_or_else(|e| {
        println!("{}", compile_error_to_message(e));
        exit(1)
    });
    println!("{:?}", opcodes);
}
