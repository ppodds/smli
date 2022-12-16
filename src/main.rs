use std::{env, fs, process::exit};

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub smli);

mod ast;
mod interpreter;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: smli <file_path>");
        exit(1);
    }
    let file_path = &args[1];
    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            println!("File not found: {}", e);
            exit(1);
        }
    };
    let parser = smli::ProgramParser::new();
    let ast_tree = match parser.parse(&content) {
        Ok(ast_tree) => ast_tree,
        Err(_) => {
            println!("syntax error");
            exit(1);
        }
    };
    let mut interpreter = interpreter::Interpreter::new();
    let success = interpreter.run(ast_tree);
    if !success {
        exit(1);
    }
}
