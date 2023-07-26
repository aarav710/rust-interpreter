use super::node::Node;

pub struct Statement {
    token_literal: String,
}

impl Node for Statement {
}

impl Statement {
    pub fn new(token_literal: String) -> Statement {
        Statement{token_literal}
    }
}
