mod lexer;
mod repl;
mod token;
mod ast;
mod parser;
mod tests;

use crate::repl::repl_start;

fn main() {
    repl_start();
}
