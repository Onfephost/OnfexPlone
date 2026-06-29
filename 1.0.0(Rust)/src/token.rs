use crate::ast::*;

#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    TyKd(TypeKind),
    Number(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Void,

    Mehen,Return,
    Urso,As,
    Valt,
    Func,    
        
    Semi,
    Star,
    And,        
    Colon,
    Minus,
    Lt,
    Gt,        
    Equal,    
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,    
    Comma,

    EOF,
}