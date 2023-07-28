use crate::token::Token;

use self::statement::Statement;

mod node;
mod expression;
mod statement;

pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new() -> Program {
        Program{statements: Vec::new()}
    }

    pub fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::from("")
        }
    }
}
