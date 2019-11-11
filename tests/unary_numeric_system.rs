extern crate expression_parser;

use expression_parser::{Operator, Operand};
use expression_parser::config::ParserConfig;
use std::fmt::Display;

mod setup;

#[derive(Debug, PartialEq)]
struct Unary(u64);

impl Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == 0u64 {
            write!(f, "0")
        } else {
            let mut s = String::with_capacity(self.0 as usize);
            for _ in 0..self.0 {
                s.push('|');
            }
            write!(f, "{}", &s)
        }
    }
}


// Unary numeric system: 0 is zero, ||||| is 5 etc..
impl Operand for Unary {

    // is zero
    type Context = bool;

    fn is_operand_partial(context: &mut Self::Context, ch: char) -> bool {
        if *context {
            return false;
        }
        if ch == '0' {
            *context = true;
            return true;
        }
        ch == '|'
    }

    fn from_str(from: &str) -> Result<Unary, ()> {
        if from.len() == 1 && from.chars().next().unwrap() == '0' {
            return Ok(Unary(0u64));
        } else {
            return Ok(Unary(from.len() as u64))
        }
    }
}

#[test]
fn basic_unary_example() {
    let mut config = ParserConfig::<Unary>::default();
    let plus =  Operator::new('+', true, |a: &Unary, b| {
        Ok(Unary(a.0 + b.0))
    }, 100);

    config.with_operator(plus);
    let num = config.parse(&"0 + ||||| + |||||||");
    assert_eq!(Ok(Unary(12u64)), num);
}