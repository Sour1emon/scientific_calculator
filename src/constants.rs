use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::{float, PRECISION, PRECISION_DIGITS};
use rug::Float;

pub fn pi() -> Float {
    Float::with_val(
        PRECISION,
        Float::parse(compute_pi::compute_pi_str(PRECISION_DIGITS as usize)).unwrap(),
    )
}

fn factorial(n: usize) -> Vec<Float> {
    let mut factorials: Vec<Float> = vec![float(1)];
    for i in 1..=n {
        factorials.push(factorials[i - 1].clone() * i);
    }
    factorials
}

pub fn e() -> Float {
    let n = PRECISION_DIGITS as usize;
    let mut e = float(2);
    let factorials = factorial(2 * n + 1);
    for i in 1..=n {
        let counter = 2 * i + 2;
        let denominator = factorials[2 * i + 1].clone();
        e += counter / denominator
    }
    e
}

lazy_static! {
    pub static ref PI: Float = pi();
    pub static ref E: Float = e();
    pub static ref CONSTANTS: HashMap<String, Float> = HashMap::from([
        ("π".to_string(), PI.clone()),
        ("pi".to_string(), PI.clone()),
        ("e".to_string(), E.clone(),)
    ]);
}