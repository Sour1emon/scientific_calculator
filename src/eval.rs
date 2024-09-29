use crate::constants::CONSTANTS;
use crate::error::CalculatorError::{DivisionByZero, ExpectedLiteral, IncorrectArguments};
use crate::functions::FUNCTIONS;
use crate::token::Token;
use anyhow::{anyhow, Result};

pub fn evaluate_postfix(tokens: Vec<Token>) -> Result<Token> {
    let mut stack: Vec<Token> = vec![];

    for token in tokens {
        if token.is_lit() {
            stack.push(token);
        } else if token.is_operator() {
            if token == Token::Minus && stack.len() == 1 {
                let val = stack.pop().unwrap();
                stack.push(-val);
                continue;
            }
            let rhs = if let Some(rhs) = stack.pop() {
                rhs
            } else {
                return Err(anyhow!(ExpectedLiteral));
            };
            let lhs = if let Some(lhs) = stack.pop() {
                lhs
            } else {
                return Err(anyhow!(ExpectedLiteral));
            };
            stack.push(if token == Token::Plus {
                lhs + rhs
            } else if token == Token::Minus {
                lhs - rhs
            } else if token == Token::Star {
                lhs * rhs
            } else if token == Token::Slash {
                if rhs.lit_val().is_zero() {
                    return Err(anyhow!(DivisionByZero));
                }
                lhs / rhs
            } else if token == Token::Caret {
                lhs.pow(rhs)
            } else {
                unimplemented!()
            });
        } else if token.is_function() {
            let func = FUNCTIONS.get(&*token.identifier()).unwrap();
            let args_len = func.1;
            if stack.len() < args_len {
                return Err(anyhow!(IncorrectArguments(stack.len(), args_len)));
            }
            let mut args: Vec<Token> = stack.drain(0..args_len).collect();
            args.reverse();
            stack.push(func.0(args))
        } else if token.is_ident() {
            stack.push(Token::Lit(
                CONSTANTS.get(&*token.identifier()).unwrap().clone(),
            ));
        } else {
            unimplemented!();
        }
    }

    Ok(stack.pop().unwrap())
}
