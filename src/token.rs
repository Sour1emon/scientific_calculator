use crate::float_to_string;
use rug::ops::Pow;
use rug::Float;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Lit(Float),
    FnCall(String),
    Identifier(String),
    Comma,
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LeftParen,
    RightParen,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Token::Lit(lit) => float_to_string(lit),
            Token::FnCall(ident) | Token::Identifier(ident) => ident.clone(),
            Token::Comma => ",".to_string(),
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Star => "*".to_string(),
            Token::Slash => "/".to_string(),
            Token::Caret => "^".to_string(),
            Token::LeftParen => "(".to_string(),
            Token::RightParen => ")".to_string(),
        })
    }
}

impl Token {
    pub fn lit_val(&self) -> Float {
        match self {
            Token::Lit(lit) => lit.clone(),
            _ => panic!("Must be int literal"),
        }
    }

    pub fn precedence(&self) -> i64 {
        match self {
            Token::Plus | Token::Minus => 2,
            Token::Star | Token::Slash => 3,
            Token::Caret => 4,
            _ => -1,
        }
    }

    pub fn is_lit(&self) -> bool {
        matches!(&self, Token::Lit(_))
    }

    pub fn is_ident(&self) -> bool {
        matches!(&self, Token::Identifier(_))
    }

    pub fn is_operator(&self) -> bool {
        matches!(
            &self,
            Token::Plus | Token::Minus | Token::Star | Token::Slash | Token::Caret
        )
    }

    pub fn is_function(&self) -> bool {
        matches!(&self, Token::FnCall(_))
    }

    pub fn left_associated(&self) -> bool {
        !matches!(&self, Token::Caret)
    }

    pub fn pow(&self, rhs: Token) -> Token {
        Token::Lit(self.lit_val().pow(rhs.lit_val()))
    }

    pub fn identifier(&self) -> String {
        match self {
            Token::Identifier(ident) | Token::FnCall(ident) => ident.clone(),
            _ => panic!("Expected function or constant"),
        }
    }
}

impl Add for Token {
    type Output = Token;

    fn add(self, rhs: Self) -> Self::Output {
        Token::Lit(self.lit_val() + rhs.lit_val())
    }
}

impl Sub for Token {
    type Output = Token;

    fn sub(self, rhs: Self) -> Self::Output {
        Token::Lit(self.lit_val() - rhs.lit_val())
    }
}

impl Mul for Token {
    type Output = Token;

    fn mul(self, rhs: Self) -> Self::Output {
        Token::Lit(self.lit_val() * rhs.lit_val())
    }
}

impl Div for Token {
    type Output = Token;

    fn div(self, rhs: Self) -> Self::Output {
        Token::Lit(self.lit_val() / rhs.lit_val())
    }
}

impl Neg for Token {
    type Output = Token;

    fn neg(self) -> Self::Output {
        Token::Lit(-self.lit_val())
    }
}
