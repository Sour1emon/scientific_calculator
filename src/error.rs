use thiserror::Error;

#[derive(Error, Debug)]
pub enum CalculatorError {
    #[error("unknown identifier `{0}`")]
    UnknownIdentifier(String),
    #[error("unknown function `{0}`")]
    UnknownFunction(String),
    #[error("unrecognized character {0:?}")]
    UnrecognizedCharacter(char),
    #[error("unmatched parentheses")]
    UnmatchedParentheses,
    #[error("expected {0} arguments found {1}")]
    IncorrectArguments(usize, usize),
    #[error("cannot divide by zero")]
    DivisionByZero,
    #[error("expected literal")]
    ExpectedLiteral
}