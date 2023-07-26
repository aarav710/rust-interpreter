use super::node::Node;

pub struct Expression {
    token_literal: String,
}

impl Node for Expression {
}

impl Expression {
    pub fn new(token_literal: String) -> Expression {
        Expression{token_literal}
    }
}
