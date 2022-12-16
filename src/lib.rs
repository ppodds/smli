#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammer);

pub mod ast;
pub mod interpreter;
mod parser_test;
