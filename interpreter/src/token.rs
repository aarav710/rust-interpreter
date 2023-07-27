pub enum Token {
    ILLEGAL(char),
    EOF,
    IDENT(String),
    INT(String),
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQ,
    NOTEQ,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    RETURN,
    IF,
    ELSE,
}

impl Token {
    pub fn call(&self) -> String {
        match self {
            Token::INT(num) => num.clone(),
            Token::ILLEGAL(value) => value.to_string(),
            Token::RETURN => String::from("return"),
            Token::LT => String::from("<"),
            Token::GT => String::from(">"),
            Token::ASSIGN => String::from("="),
            Token::FUNCTION => String::from("fn"),
            Token::LET => String::from("let"),
            Token::IDENT(value) => value.clone(),
            Token::SEMICOLON => String::from(";"),
            _ => String::from(""),
        }
    }
}

pub fn convert_to_token(command: &str) -> Token {
   match command {
        "let" => Token::LET,
        "fn" => Token::FUNCTION,
        "false" => Token::FALSE,
        "true" => Token::TRUE,
        "if" => Token::IF,
        "else" => Token::ELSE,
        "return" => Token::RETURN,
        _ => Token::IDENT(command.to_string()),
   }
}
