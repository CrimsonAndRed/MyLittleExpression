use crate::Operator;
use crate::Operand;
use crate::ParserConfig;

impl ParserConfig<i64> {
    pub fn with_basic_math(&mut self) {
        let plus = Operator::new('+', true, |a: &i64, b| a.checked_add(*b).ok_or_else(|| format!("Add with overflow: {} + {}", a, b)), 100);
        let minus = Operator::new('-', true, |a: &i64, b| a.checked_sub(*b).ok_or_else(|| format!("Subtract with overflow: {} - {}", a, b)), 100);
        let multiply = Operator::new('*', true, |a: &i64, b| a.checked_mul(*b).ok_or_else(|| format!("Multiplication with overflow: {} * {}", a, b)), 200);
        let divide = Operator::new('/', true, |a: &i64, b| a.checked_div(*b).ok_or_else(|| format!("Division with overflow: {} / {}", a, b)), 200);
        let power = Operator::new('^', false, |a: &i64, b| {
            if *b < 0 {
                return Err(format!("Could not calculate negative exponent as integer: {} ^ {}", a, b));
            }

            if *b > std::u32::MAX as i64 {
                return Err(format!("Too big exponent: {} ^ {}", a, b));
            }
            i64::checked_pow(*a, *b as u32).ok_or_else(|| format!("Power with overflow: {} ^ {}", a, b))
        }, 300);

        self.operators.insert(plus.symbol, plus);
        self.operators.insert(minus.symbol, minus);
        self.operators.insert(multiply.symbol, multiply);
        self.operators.insert(divide.symbol, divide);
        self.operators.insert(power.symbol, power);
    }
}

impl Operand for i64 {
    fn parse_operand(from: &[char]) -> Option<(usize, Self)> {
        let mut index = 0usize;
        let mut res = 0i64;

        while index < from.len() {
            match from[index].to_digit(10) {
                Some(digit) => {
                    res = res * 10 + digit as i64;
                    index += 1;
                    continue;
                },
                None => break,
            }
        }

        if index == 0 {
            None
        } else {
            Some((index, res))
        }
    }
}