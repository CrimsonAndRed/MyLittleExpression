pub mod config;

mod parser;

use std::collections::VecDeque;

use config::ParserConfig;
use parser::RPNToken;

impl ParserConfig {
    pub fn parse(&self, formula: &str) -> Result<i64, ExprError> {

        let yard = self.yard_from_str(formula)?;

        let mut rpn: VecDeque<i64> = VecDeque::new();

        for token in yard.iter() {
            match token {
                RPNToken::Number(num) => rpn.push_back(*num),
                RPNToken::Operator(op) => {
                    let arg2 = rpn.pop_back().ok_or_else(|| ExprError::IncorrectToken(format!("Not enough arguments for operator {}", op.symbol)))?;
                    let arg1 = rpn.pop_back().ok_or_else(|| ExprError::IncorrectToken(format!("Not enough arguments for operator {}", op.symbol)))?;
                    rpn.push_back(
                        (op.operation)(arg1, arg2)
                    );
                },
            }
        }

        let res = rpn.pop_back().unwrap();
        assert!(rpn.is_empty());
        Ok(res)
    }
}

#[derive(Debug)]
pub struct Operator {
    pub(crate) symbol: char,
    pub(crate) left_associative: bool,
    pub(crate) operation: fn(i64, i64) -> i64,
    pub(crate) order: u64
}

impl Operator {
    pub fn new(symbol: char, left_associative: bool, operation: fn(i64, i64) -> i64, order: u64) -> Self {
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
    IncorrectToken(String),
    ArithmeticsException(String),
}

impl std::fmt::Display for ExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ExprError::IncorrectToken(s) => write!(f, "Incorrect token: {}", s),
            ExprError::ArithmeticsException(s) => write!(f, "Arithmetics failed at {}", s),
        }
    }
}

impl std::error::Error for ExprError {}
