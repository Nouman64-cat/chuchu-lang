pub mod ast;
pub mod lexer;
pub mod parser;
pub mod symbol_table;
pub mod token;
pub mod type_checker;
pub mod types;

pub fn run() {
    println!("Chuchu compiler initialized");
}
