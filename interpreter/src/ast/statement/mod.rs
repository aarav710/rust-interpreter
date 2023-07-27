mod let_statement;


pub trait Statement {
    fn statement_node(&self);
    fn token_literal(&self) -> String;
}
