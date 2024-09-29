#![feature(let_chains)]

mod constants;
mod error;
mod eval;
mod functions;
mod number;
mod parser;
mod token;

macro_rules! unwrap_or_print_err {
    ($x:expr) => {
        match $x {
            Ok(ok) => ok,
            Err(err) => {
                println!("{err}");
                continue;
            }
        }
    };
}

use crate::eval::evaluate_postfix;
use crate::parser::{infix_to_postfix, tokenize};
use crate::token::Token;
use anyhow::Result;
use rug::{float, Assign, Float};
use std::f64::consts::LOG2_10;
use std::io;
use std::io::{stdout, Write};

fn format_tokens(tokens: &[Token]) -> String {
    Vec::from_iter(tokens.iter().map(|i| i.to_string()))
        .join(" ")
        .to_string()
}

fn float<T>(val: T) -> Float
where
    Float: Assign<T>,
{
    Float::with_val(PRECISION, val)
}

const PRECISION_DIGITS: u32 = 100;
const PRECISION: u32 = ((PRECISION_DIGITS as f64) * LOG2_10) as u32 + 10;

pub fn float_to_string(float: &Float) -> String {
    if float.is_zero() {
        return "0".to_string();
    }
    let mut float_string = float.to_string_radix(10, Some(PRECISION_DIGITS as usize));

    let float_string_clone = float_string.clone();

    let mut out_reverse = float_string_clone.chars().rev().peekable();

    while let Some('0') = out_reverse.peek() {
        out_reverse.next().unwrap();
        float_string.pop().unwrap();
    }

    if let Some('.') = out_reverse.next() {
        float_string.pop().unwrap();
    }

    float_string
}

fn main() -> Result<()> {
    let mut input = String::new();
    let stdin = io::stdin();
    loop {
        print!("> ");
        stdout().flush()?;
        input.clear();
        stdin.read_line(&mut input).expect("Failed to read input");
        input = input.trim().to_string();

        if input == *"exit" {
            return Ok(());
        } else if input == *"clear" {
            print!("{esc}c", esc = 27 as char);
            continue;
        }

        let tokens = unwrap_or_print_err!(tokenize(&input));
        let infix_tokens = unwrap_or_print_err!(infix_to_postfix(tokens));
        let result = unwrap_or_print_err!(evaluate_postfix(infix_tokens));
        println!("{}", result);
    }
}
