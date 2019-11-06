pub mod config;

mod parser;
mod error;

use std::collections::VecDeque;
use error::ExprError;

use config::ParserConfig;
use parser::RPNToken;

impl<'a> ParserConfig<i64> {
    pub fn parse(&self, formula: &'a str) -> Result<i64, ExprError<'a>> {

        let yard = self.yard_from_str(formula)?;

        let mut rpn: VecDeque<i64> = VecDeque::new();

        for token in yard.iter() {
            match token {
                RPNToken::Number(num) => rpn.push_back(*num),
                RPNToken::Operator(op) => {
                    let arg2 = rpn.pop_back().ok_or_else(|| ExprError::Unexpected(format!("Not enough arguments for operator {}", op.symbol)))?;
                    let arg1 = rpn.pop_back().ok_or_else(|| ExprError::Unexpected(format!("Not enough arguments for operator {}", op.symbol)))?;
                    rpn.push_back(
                        (op.operation)(arg1, arg2).map_err(|e| ExprError::ArithmeticException(format!("Error during computation {} over arguments {} and {}:\n{}", &op.symbol, &arg1, &arg2, &e)))?
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