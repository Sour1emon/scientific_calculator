use crate::error::CalculatorError::{UnmatchedParentheses, UnrecognizedCharacter};
use crate::float;
use crate::token::Token;
use anyhow::{anyhow, Result};
use rug::Float;

pub fn infix_to_postfix(tokens: Vec<Token>) -> Result<Vec<Token>> {
    let mut output: Vec<Token> = vec![];
    let mut stack: Vec<Token> = vec![];

    for token in tokens {
        if token.is_lit() || token.is_ident() {
            output.push(token);
        } else if token.is_function() {
            stack.push(token);
        } else if token.is_operator() {
            while let Some(last) = stack.last()
                && last != &Token::LeftParen
                && (last.precedence() > token.precedence()
                    || (last.precedence() == token.precedence() && token.left_associated()))
            {
                output.push(stack.pop().unwrap());
            }
            stack.push(token);
        } else if token == Token::Comma {
            while let Some(last) = stack.last()
                && last != &Token::LeftParen
            {
                output.push(stack.pop().unwrap());
            }
        } else if token == Token::LeftParen {
            stack.push(token);
        } else if token == Token::RightParen {
            while let Some(last) = stack.last()
                && last != &Token::LeftParen
            {
                output.push(stack.pop().unwrap());
            }
            if stack.pop().unwrap() != Token::LeftParen {
                return Err(anyhow!(UnmatchedParentheses));
            }
            if let Some(last) = stack.last()
                && last.is_function()
            {
                output.push(stack.pop().unwrap());
            }
        }
    }

    while let Some(token) = stack.pop() {
        if token == Token::LeftParen {
            return Err(anyhow!(UnmatchedParentheses));
        }
        output.push(token);
    }

    Ok(output)
}

pub fn tokenize(str: &str) -> Result<Vec<Token>> {
    let mut tokens = vec![];
    let mut chars = str.chars().peekable();

    while let Some(c) = chars.next() {
        if c.is_whitespace() {
            continue;
        } else if c.is_alphabetic() {
            let mut identifier = String::from(c);
            while let Some(ch) = chars.peek()
                && ch.is_ascii_alphabetic()
            {
                identifier.push(chars.next().unwrap())
            }
            if let Some(&next) = chars.peek()
                && next == '('
            {
                tokens.push(Token::FnCall(identifier));
            } else {
                tokens.push(Token::Identifier(identifier));
            }
        } else {
            match c {
                '0'..='9' => {
                    let mut num = String::from(c);
                    let mut had_dot = false;
                    while let Some(&next_c) = chars.peek()
                        && (next_c.is_ascii_digit() || (next_c == '.' && !had_dot))
                    {
                        if next_c == '.' {
                            had_dot = true;
                        }
                        num.push(chars.next().unwrap());
                    }
                    tokens.push(Token::Lit(float(Float::parse(num)?)))
                }
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Star),
                '/' => tokens.push(Token::Slash),
                '^' => tokens.push(Token::Caret),
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                ',' => tokens.push(Token::Comma),
                _ => return Err(anyhow!(UnrecognizedCharacter(c))),
            }
        }
    }

    Ok(tokens)
}
