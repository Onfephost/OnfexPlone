use crate::token::Token;
use crate::ast::*;
pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {

    pub fn new(code: String) -> Self {
        Self {
            chars: code.chars().collect(),
            pos: 0,
        }
    }

    fn skip(&mut self) {
        while self.pos < self.chars.len()
            && self.chars[self.pos].is_whitespace()
        {
            self.pos += 1;
        }
    }

    pub fn next_token(&mut self) -> Token {

        self.skip();

        if self.pos >= self.chars.len() {
            return Token::EOF;
        }

        let ch = self.chars[self.pos];

        match ch {

            '(' => { self.pos += 1; Token::LParen }
            ')' => { self.pos += 1; Token::RParen }
            '['=>{self.pos+=1;Token::LBracket}
            ']'=>{self.pos+=1;Token::RBracket}
            '{' => { self.pos += 1; Token::LBrace }
            '}' => { self.pos += 1; Token::RBrace }
            ',' => { self.pos += 1; Token::Comma }
            ';' => { self.pos += 1; Token::Semi }
            '=' => { self.pos += 1; Token::Equal }
            ':' => { self.pos += 1; Token::Colon }
            '-' => { self.pos += 1; Token::Minus }
            '>' => { self.pos += 1; Token::Gt }
            '<' => { self.pos += 1; Token::Lt }
            '*' => { self.pos += 1; Token::Star }
            '&' => { self.pos += 1; Token::And }

            '0'..='9' => {
                let mut n = String::new();
                let mut is_float = false;

                while self.pos < self.chars.len()
                    && (self.chars[self.pos].is_numeric()
                        || self.chars[self.pos] == '.')
                {
                    if self.chars[self.pos] == '.' {
                        is_float = true;
                    }

                    n.push(self.chars[self.pos]);
                    self.pos += 1;
                }

                if is_float {
                    Token::Float(n.parse().unwrap())
                } else {
                    Token::Number(n.parse().unwrap())
                }
            }

            '"' => {
                self.pos += 1;
                let mut s = String::new();

                while self.pos < self.chars.len()
                    && self.chars[self.pos] != '"'
                {
                    s.push(self.chars[self.pos]);
                    self.pos += 1;
                }

                self.pos += 1;
                Token::String(s)
            }

            'a'..='z' | 'A'..='Z' => {
                let mut w = String::new();

                while self.pos < self.chars.len()
                    && self.chars[self.pos].is_alphanumeric()
                {
                    w.push(self.chars[self.pos]);
                    self.pos += 1;
                }
                if let Some(i) = TypeKind::from_string(w.clone().as_str()){
                    return Token::TyKd(i);
                }
                match w.as_str() {
                    "mehen" => Token::Mehen,"frnct" => Token::Func,
                    "urso" => Token::Urso,"erutnos" => Token::Return,
                    "valt" => Token::Valt,"asken" => Token::As,
                    "trunth" => Token::Bool(true),
                    "frunth" => Token::Bool(false),
                    "noph" => Token::Void,
                    _ => Token::Identifier(w),
                }
            }

            _ => panic!("invalid char"),
        }
    }
    
}