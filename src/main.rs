#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub smli);

mod ast;
mod interpreter;
mod parser;

fn main() {}
