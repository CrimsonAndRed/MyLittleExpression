extern crate expression_parser;

pub mod setup;

use expression_parser::{Operator, Operand};
use expression_parser::config::ParserConfig;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
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
    fn parse_operand(from: &[char]) -> Option<(usize, Self)> {
        let mut index = 0usize;
        if from.is_empty() {
            return None;
        }

        if from[0] == '0' {
            return Some((1, Unary(0)));
        }


        while index < from.len() {
            if from[index] == '|' {
                index += 1;
            } else {
                break;
            }
        }

        if index == 0 {
            return None;
        } else {
            return Some((index, Unary(index as u64)));
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