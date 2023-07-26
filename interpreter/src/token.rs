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
    pub fn call(&self) {

    }

    pub fn print_value(&self) {
        match self {
            Token::INT(num) => println!("{}", num),
            Token::ILLEGAL(value) => println!("{}", value),
            _ => println!("other data type"),
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
