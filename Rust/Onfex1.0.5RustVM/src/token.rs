use crate::ast::*;


#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    Identifier(String),
    TyKd(TypeKind),
    Number(i64),
    Float(f64),
    String(String),
    Bool(bool),
    TypeL,
    TypeM,
    Void,
    Mehen,
    Return,
    Urso,
    Mod,elsnt,ifnt,      
    As,
    Valt,
    Func,Strct,Priv,Pub,Impl,
    Semi,
    Star,
    Clam,    

    And,

    Colon,

    Minus,Plus,Div,Perc,

    Lt,
    Gt,

    Equal,DEqual,
        
    LParen,
    RParen,

    LBrace,
    RBrace,

    LBracket,
    RBracket,
        
    Comma,
    Dot,

    EOF,

}