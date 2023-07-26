mod lexer;
mod repl;
mod token;
mod ast;

use crate::repl::repl_start;

fn main() {
    repl_start();
}
