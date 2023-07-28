use crate::{token::Token, ast::{expression::Expression, node::{Node, Identifier}}};

use super::Statement;

pub struct LetStatement {
    token: Token,
    name: Identifier,
    value: Box<dyn Expression>,
}

impl Statement for LetStatement {
    fn statement_node(&self) {

    }

    fn token_literal(&self) -> String {
        self.token.call()
    }
}

impl LetStatement {
    pub fn new(token: Token, name: Identifier, value: Box<dyn Expression>) -> LetStatement {
        LetStatement{token, name, value}
    }
}
