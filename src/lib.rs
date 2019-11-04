pub mod config;

mod parser;

use std::collections::VecDeque;

use config::ParserConfig;
use parser::RPNToken;

impl ParserConfig<i64> {
    pub fn parse(&self, formula: &str) -> Result<i64, ExprError> {

        let yard = self.yard_from_str(formula)?;

        let mut rpn: VecDeque<i64> = VecDeque::new();

        for token in yard.iter() {
            match token {
                RPNToken::Number(num) => rpn.push_back(*num),
                RPNToken::Operator(op) => {
                    let arg2 = rpn.pop_back().ok_or_else(|| ExprError::Unexpected(format!("Not enough arguments for operator {}", op.symbol)))?;
                    let arg1 = rpn.pop_back().ok_or_else(|| ExprError::Unexpected(format!("Not enough arguments for operator {}", op.symbol)))?;
                    rpn.push_back(
                        (op.operation)(arg1, arg2).map_err(|e| ExprError::ArithmeticsException(format!("Error during computation {} over arguments {} and {}: {}", &op.symbol, &arg1, &arg2, &e)))?
                    );
                },
            }
        }

        let res = rpn.pop_back().ok_or_else(|| ExprError::Unexpected("Could not get solution because RPN stack was too short".to_owned()))?;
        if !rpn.is_empty() {
            return Err(ExprError::Unexpected(format!("Processed until solution but there was {} more items in RPN queue", rpn.len())));
        }
        Ok(res)
    }
}

#[derive(Debug)]
pub struct Operator<T> {
    pub(crate) symbol: char,
    pub(crate) left_associative: bool,
    pub(crate) operation: fn(T, T) -> Result<T, String>,
    pub(crate) order: u64
}

impl <T> Operator<T> {
    pub fn new(symbol: char, left_associative: bool, operation: fn(T, T) -> Result<T, String>, order: u64) -> Self {
        Operator {
            symbol,
            left_associative,
            operation,
            order
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExprError {
    IncorrectToken(String, usize),
    ArithmeticsException(String),
    Unexpected(String),
}

impl std::fmt::Display for ExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ExprError::IncorrectToken(i, s) => write!(f, "Incorrect token: {} at index {}", &s, i),
            ExprError::ArithmeticsException(s) => write!(f, "Arithmetics failed at {}", &s),
            ExprError::Unexpected(s) => write!(f, "Unexpected error: {}.\n You'd better report it", &s)
        }
    }
}

impl std::error::Error for ExprError {}
