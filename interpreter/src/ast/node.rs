use crate::token::Token;

use super::{statement::Statement, expression::Expression};

pub enum Node {
    Statement(Box<dyn Statement>),
    Expression(Box<dyn Expression>),
}

pub struct Identifier {
    token: Token,
}

impl Identifier {
    pub fn expression_node() {

    }

    pub fn token_literal(&self) -> String {
        self.token.call().to_string()
    }
}
