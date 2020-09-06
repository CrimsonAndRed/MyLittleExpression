#[cfg(feature = "bigint")]
use crate::Operator;
#[cfg(feature = "bigint")]
use crate::Operand;
#[cfg(feature = "bigint")]
use crate::ParserConfig;
#[cfg(feature = "bigint")]
use num_bigint::BigInt;
#[cfg(feature = "bigint")]
use std::str::FromStr;

#[cfg(feature = "bigint")]
impl ParserConfig<BigInt> {
    pub fn with_basic_math(&mut self) {
        let plus = Operator::new('+', true, |a: &BigInt, b| Ok(a + b), 100);
        let minus = Operator::new('-', true, |a: &BigInt, b| Ok(a - b), 100);
        let multiply = Operator::new('*', true, |a: &BigInt, b| Ok(a * b), 200);
        let divide = Operator::new('/', true, |a: &BigInt, b| Ok(a / b), 200);

        self.operators.insert(plus.symbol, plus);
        self.operators.insert(minus.symbol, minus);
        self.operators.insert(multiply.symbol, multiply);
        self.operators.insert(divide.symbol, divide);
    }
}

#[cfg(feature = "bigint")]
impl Operand for BigInt {
    fn parse_operand(from: &[char]) -> Option<(usize, Self)> {
        let mut index = 0usize;

        while index < from.len() {
            match from[index].to_digit(10) {
                Some(digit) => {
                    index += 1;
                    continue;
                },
                None => break,
            }
        }

        if index == 0 {
            None
        } else {
            // TODO no allocation possible?
            BigInt::from_str(&(from[0..index].iter().collect::<String>())).ok().map(|res| (index, res))
        }
    }
}