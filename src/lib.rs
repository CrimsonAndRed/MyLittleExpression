pub mod config;

mod parser;
mod error;

use std::collections::VecDeque;
use error::ExprError;

use config::ParserConfig;
use parser::RPNToken;

impl<'a, T> ParserConfig<T> where T: Operand {
    pub fn parse(&self, formula: &'a str) -> Result<T, ExprError<'a>> {

        let mut yard = self.yard_from_str(formula)?;

        let mut rpn: VecDeque<T> = VecDeque::new();

        for token in yard.drain(..) {
            match token {
                RPNToken::Operand(num) => rpn.push_back(num),
                RPNToken::Operator(op) => {
                    let arg2 = rpn.pop_back().ok_or_else(|| ExprError::Unexpected(format!("Not enough arguments for operator {}", op.symbol)))?;
                    let arg1 = rpn.pop_back().ok_or_else(|| ExprError::Unexpected(format!("Not enough arguments for operator {}", op.symbol)))?;
                    // Pass arguments by reference - is it ok?
                    rpn.push_back(
                        (op.operation)(&arg1, &arg2).map_err(|e| ExprError::ArithmeticException(format!("Error during computation {} over arguments {} and {}:\n{}", &op.symbol, &arg1, &arg2, &e)))?
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

pub struct Operator<T> where T: Operand {
    pub(crate) symbol: char,
    pub(crate) left_associative: bool,
    pub(crate) operation: fn(&T, &T) -> Result<T, String>,
    pub(crate) order: u64
}

impl <T> Operator<T> where T: Operand {
    pub fn new(symbol: char, left_associative: bool, operation: fn(&T, &T) -> Result<T, String>, order: u64) -> Self {
        Operator {
            symbol,
            left_associative,
            operation,
            order
        }
    }
}

// TODO should not be sized, but how should i return Self? Box<i64>?
pub trait Operand: std::fmt::Display + Sized {
    type Context: Default;

    fn is_operand_partial(context: &mut Self::Context, ch: char) -> bool;
    fn from_str(from: &str) -> Result<Self, ()>;
}

impl Operand for i64 {

    type Context = ();

    fn is_operand_partial(_context: &mut Self::Context, ch: char) -> bool {
        ch.is_digit(10)
    }

    fn from_str(from: &str) -> Result<i64, ()> {
        i64::from_str_radix(&from, 10).map_err(|_| {})
    }
}

impl Operand for f64 {

    // had already parsed dot
    type Context = bool;

    fn is_operand_partial(context: &mut Self::Context, ch: char) -> bool {

        if ch.is_digit(10) {
            return true;
        }

        if ch == '.' && !*context {
            *context = true;
            return true;
        }

        false
    }

    fn from_str(from: &str) -> Result<f64, ()> {
        from.parse::<f64>().map_err(|_| {})
    }
}