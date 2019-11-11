pub mod config;

mod parser;
mod error;
mod small_integer;
mod small_float;

use std::collections::VecDeque;
use error::ExprError;

use config::ParserConfig;
use parser::RPNToken;

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