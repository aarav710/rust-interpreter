use crate::lexer::Lexer;
use crate::token::Token;

pub fn repl_start() {
    loop {
        println!(">>");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut lexer = Lexer::new(input);
        let mut token = lexer.next_token();
        while !matches!(token, Token::EOF) {
            println!("{}", token.call());
            token = lexer.next_token();
        }
    }
}
