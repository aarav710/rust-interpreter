use crate::token::{Token, convert_to_token};

pub struct Lexer {
    input: String,
    position: u32,
    current_char: char,
    read_position: u32,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
       let mut lexer = Lexer{input, position: 0, read_position: 0, current_char:'\0'};
       lexer.read_char();
       return lexer;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let mut token: Token = Token::ASSIGN;
        match self.current_char {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    token = Token::EQ;
                } else {
                    token = Token::ASSIGN;
                }
            }
            ';' => token = Token::SEMICOLON,
            '+' => token = Token::PLUS,
            '(' => token = Token::LPAREN,
            ')' => token = Token::RPAREN,
            '{' => token = Token::LBRACE,
            '}' => token = Token::RBRACE,
            ',' => token = Token::COMMA,
            '-' => token = Token::MINUS,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    token = Token::NOTEQ;
                } else {
                    token = Token::BANG;
                }
            }
            '>' => token = Token::GT,
            '<' => token =  Token::LT,
            '*' => token = Token::ASTERISK,
            '/' => token = Token::SLASH,
            '\0' => token =  Token::EOF,
            _ => {
                if self.current_char.is_alphabetic() {
                    token = convert_to_token(self.read_identifier().as_str());
                    return token;
                } else if self.current_char.is_digit(10) {
                    token = Token::INT(self.read_number());
                    return token;
                } else {
                    token = Token::ILLEGAL(self.current_char);
                }
            }
        }
        self.read_char();
        return token;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() as u32 {
            self.current_char = '\0';
        } else {
            self.current_char = self.input.as_bytes()[self.read_position as usize] as char;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn skip_whitespace(&mut self) {
        while self.current_char == ' ' || self.current_char == '\t' ||
            self.current_char == '\n' || self.current_char == '\r' {
                self.read_char();
        }
    }

    pub fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() as u32 {
            return '\0';
        } else {
            return self.input.as_bytes()[self.read_position as usize] as char;
        }
    }

    pub fn read_number(&mut self) -> String {
        let temp_position: u32 = self.position;
        while self.current_char.is_digit(10) {
            self.read_char();
        }
        return self.input.chars()
            .skip(temp_position as usize)
            .take((self.position - temp_position) as usize)
            .collect();
    }

    pub fn read_identifier(&mut self) -> String {
        let temp_position: u32 = self.position;
        while self.current_char.is_alphabetic() {
            self.read_char();
        }
        return self.input.chars()
            .skip(temp_position as usize)
            .take((self.position - temp_position) as usize)
            .collect();
    }
}
