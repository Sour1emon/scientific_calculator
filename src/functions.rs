use crate::constants::PI;
use crate::float;
use crate::token::Token;
use phf::{phf_map, Map};
use rand::random;
use rug::rand::RandState;
use rug::{Float, Integer};

type FunctionDef = fn(args: Vec<Token>) -> Token;

pub static FUNCTIONS: Map<&str, (FunctionDef, usize)> = phf_map! {
    "sin" => (|args| {
        Token::Lit(args.first().unwrap().lit_val().sin())
    }, 1),
    "sinh" => (|args| {
        Token::Lit(args.first().unwrap().lit_val().sinh())
    }, 1),
    "cos" => (|args| {
        Token::Lit(args.first().unwrap().lit_val().cos())
    }, 1),
    "cosh" => (|args| {
        Token::Lit(args.first().unwrap().lit_val().cosh())
    }, 1),
    "tan" => (|args| {
        Token::Lit(args.first().unwrap().lit_val().tan())
    }, 1),
    "tanh" => (|args| {
        Token::Lit(args.first().unwrap().lit_val().tanh())
    }, 1),
    "min" => (|args| {
        let arg1 = args.first().unwrap();
        let arg2 = args.get(1).unwrap();
        Token::Lit(arg1.lit_val().min(&arg2.lit_val()))
    }, 2),
    "max" => (|args| {
        let arg1 = args.first().unwrap();
        let arg2 = args.get(1).unwrap();
        Token::Lit(arg1.lit_val().max(&arg2.lit_val()))
    }, 2),
    "rand" => (|_args| {
        let mut rand_state = RandState::new();
        rand_state.seed(&Integer::from(random::<usize>()));
        Token::Lit(float(Float::random_bits(&mut rand_state)))
    }
    , 0),
    "rad" => (|args| {
        Token::Lit(args.first().unwrap().lit_val() * PI.clone() / 180)
    }, 1),
    "deg" => (|args| {
        Token::Lit(args.first().unwrap().lit_val() / (PI.clone() / 180))
    }, 1),
    "sqrt" => (|args| {
        Token::Lit(args.first().unwrap().lit_val().sqrt())
    }, 1)
};
