pub mod config;

mod parser;
mod error;
mod small_integer;
mod small_float;
mod big_integer;

use error::ExprError;

use config::ParserConfig;

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
    fn parse_operand(from: &[char]) -> Option<(usize, Self)>;
}