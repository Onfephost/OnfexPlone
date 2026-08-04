use crate::token::{Token, TokenKind};
use crate::ast::*;
use crate::error::OnfexError;

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Self {
            chars: code.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn advance(&mut self) {
        if self.pos < self.chars.len() {
            if self.chars[self.pos] == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            self.pos += 1;
        }
    }

    fn skip(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
            self.advance();
        }
    }

    pub fn next_token(&mut self) -> Result<Token, OnfexError> {
        self.skip();
        let line = self.line;
        let col = self.col;
        if self.pos >= self.chars.len() {
            return Ok(Token {
                kind: TokenKind::EOF,
                line,
                col,
            });
        }
        let ch = self.chars[self.pos];
        let make = |kind| Token { kind, line, col };
        match ch {
            '(' => {
                self.advance();
                Ok(make(TokenKind::LParen))
            }
            ')' => {
                self.advance();
                Ok(make(TokenKind::RParen))
            }
            '[' => {
                self.advance();
                Ok(make(TokenKind::LBracket))
            }
            ']' => {
                self.advance();
                Ok(make(TokenKind::RBracket))
            }
            '{' => {
                self.advance();
                Ok(make(TokenKind::LBrace))
            }
            '}' => {
                self.advance();
                Ok(make(TokenKind::RBrace))
            }
            ',' => {
                self.advance();
                Ok(make(TokenKind::Comma))
            }
            '.' => {
                self.advance();
                Ok(make(TokenKind::Dot))
            }
            ';' => {
                self.advance();
                Ok(make(TokenKind::Semi))
            }
            '=' => {
                self.advance();
                let chk = self.chars[self.pos];
                if chk == '='{
                    self.advance();
                    Ok(make(TokenKind::DEqual))
                }else{
                    Ok(make(TokenKind::Equal))
                }
                
            }
            ':' => {
                self.advance();
                Ok(make(TokenKind::Colon))
            }
            '-' => {
                self.advance();
                Ok(make(TokenKind::Minus))
            }
            '+' => {
                self.advance();
                Ok(make(TokenKind::Plus))
            }
            '/' => {
                self.advance();
                Ok(make(TokenKind::Div))
            }
            '%' => {
                self.advance();
                Ok(make(TokenKind::Perc))
            }
            '>' => {
                self.advance();
                Ok(make(TokenKind::Gt))
            }
            '<' => {
                self.advance();
                Ok(make(TokenKind::Lt))
            }
            '*' => {
                self.advance();
                Ok(make(TokenKind::Star))
            }
            '&' => {
                self.advance();
                Ok(make(TokenKind::And))
            }
            '!' => {
                self.advance();
                Ok(make(TokenKind::Clam))
            }
            '0'..='9' => {
                let mut n = String::new();
                let mut float = false;
                while self.pos < self.chars.len()
                    && (self.chars[self.pos].is_numeric() || self.chars[self.pos] == '.')
                {
                    if self.chars[self.pos] == '.' {
                        float = true;
                    }
                    n.push(self.chars[self.pos]);
                    self.advance();
                }

                if float {
                    match n.parse::<f64>() {
                        Ok(v) => Ok(make(TokenKind::Float(v))),
                        Err(_) => Err(OnfexError::lexer(
                            format!("invalid float literal '{}'", n),
                            line,
                            col,
                        )),
                    }
                } else {
                    match n.parse::<i64>() {
                        Ok(v) => Ok(make(TokenKind::Number(v))),
                        Err(_) => Err(OnfexError::lexer(
                            format!("invalid integer literal '{}'", n),
                            line,
                            col,
                        )),
                    }
                }
            }
            '"' => {
                self.advance();
                let mut s = String::new();
                while self.pos < self.chars.len() && self.chars[self.pos] != '"' {
                    s.push(self.chars[self.pos]);
                    self.advance();
                }
                if self.pos >= self.chars.len() {
                    return Err(OnfexError::lexer("unterminated string", line, col));
                }
                self.advance();
                Ok(make(TokenKind::String(s)))
            }
            'a'..='z' | 'A'..='Z' => {
                let mut w = String::new();
                while self.pos < self.chars.len() && self.chars[self.pos].is_alphanumeric() {
                    w.push(self.chars[self.pos]);
                    self.advance();
                }
                let kind = if let Some(i) = TypeKind::from_string(&w) {
                    TokenKind::TyKd(i)
                } else {
                    match w.as_str() {
                        "mehen" => TokenKind::Mehen,
                        "frounct" => TokenKind::Func,
                        "strouct" => TokenKind::Strct,
                        "urso" => TokenKind::Urso,
                        "mot" => TokenKind::Mod,
                        "erutnos" => TokenKind::Return,
                        "ifnt" => TokenKind::ifnt,
                        "prube" => TokenKind::Pub,
                        "prive" => TokenKind::Priv,
                            "impelnos" => TokenKind::Impl,       
                        "elsnt" => TokenKind::elsnt,
                        "wrossnosLrib" => TokenKind::TypeL,
                        "wrossnosMot" => TokenKind::TypeM,
                            "valt" => TokenKind::Valt,   
                        "asken" => TokenKind::As,
                        "trunth" => TokenKind::Bool(true),
                        "frunth" => TokenKind::Bool(false),
                        "noph" => TokenKind::Void,
                        _ => TokenKind::Identifier(w),
                    }
                };
                Ok(make(kind))
            }
            _ => Err(OnfexError::lexer(format!("invalid character '{}'", ch), line, col)),
        }
    }
}
