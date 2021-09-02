// mod宣言をしなくてもファイルがモジュールになる模様
use std::fmt;
use std::fmt::Formatter;


pub fn sum_in_module(a: i32, b: i32) -> i32 {
    return a + b;
}

type TokenType = String;

pub struct Token {
    pub cat: TokenType,
    pub literal: String,
}

pub enum Cat {
    ILLEGAL,
    EOF,

    IDENT,
    INT,

    ASSIGN,
    PLUS,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Cat::ILLEGAL => write!(f, "ILLEGAL"),
            Cat::EOF => write!(f, "EOF"),
            Cat::IDENT => write!(f, "IDENT"),
            Cat::INT => write!(f, "INT"),
            Cat::ASSIGN => write!(f, "ASSIGN"),
            Cat::PLUS => write!(f, "PLUS"),
            Cat::COMMA => write!(f, "COMMA"),
            Cat::SEMICOLON => write!(f, "SEMICOLON"),
            Cat::LPAREN => write!(f, "LPAREN"),
            Cat::RPAREN => write!(f, "RPAREN"),
            Cat::LBRACE => write!(f, "LBRACE"),
            Cat::RBRACE => write!(f, "RBRACE"),
            Cat::FUNCTION => write!(f, "FUNCTION"),
            Cat::LET => write!(f, "LET"),
            _ => write!(f, "unknown token")
        }
    }
}