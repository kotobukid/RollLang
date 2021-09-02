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

#[derive(Debug)]
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

// impl fmt::Display for Cat {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self)
//     }
// }