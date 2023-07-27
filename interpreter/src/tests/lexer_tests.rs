
#[cfg(test)]
mod lexer_tests {
    use crate::{token::Token, lexer::Lexer};

    #[test]
    fn lexer_token_check() {
        let input: String = String::from("{=})");
        let expect: Vec<Token> = vec![Token::LBRACE, 
        Token::ASSIGN, 
        Token::RBRACE, 
        Token::RPAREN];
        let mut curr_tokens: Vec<Token> = Vec::new();
        let mut lexer = Lexer::new(input);
        let mut token = lexer.next_token();
        curr_tokens.push(token);
        while !matches!(&curr_tokens.last().unwrap(), Token::EOF) {
            token = lexer.next_token();
            curr_tokens.push(token);
        }
        for i in 0..expect.len() {
            assert_eq!(expect.get(i), curr_tokens.get(i));
        }
    }
}
