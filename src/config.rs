use std::collections::HashMap;
use crate::Operator;
use crate::Operand;

pub struct ParserConfig<T> where T: Operand {
    pub(crate) operators: HashMap<char, Operator<T>>,
}

impl<T> Default for ParserConfig<T> where T: Operand {
    fn default() -> Self {
        let mut parser = ParserConfig {
            operators: HashMap::new(),
        };

        parser
    }
}

impl ParserConfig<i64> {
    pub fn with_basic_math(&mut self) {
        let plus = Operator::new('+', true, |a: &i64, b| a.checked_add(*b).ok_or_else(|| format!("Add with overflow: {} + {}", a, b)), 100);
        let minus = Operator::new('-', true, |a: &i64, b| a.checked_sub(*b).ok_or_else(|| format!("Subtract with overflow: {} - {}", a, b)), 100);
        let multiply = Operator::new('*', true, |a: &i64, b| a.checked_mul(*b).ok_or_else(|| format!("Multiplication with overflow: {} * {}", a, b)), 200);
        let divide = Operator::new('/', true, |a: &i64, b| a.checked_div(*b).ok_or_else(|| format!("Division with overflow: {} / {}", a, b)), 200);
        let power = Operator::new('^', false, |a: &i64, b| {
            if *b < 0 {
                return Err(format!("Could not calculate negative exponent: {} ^ {}", a, b));
            }

            if *b > std::u32::MAX as i64 {
                return Err(format!("Too big exponent: {} ^ {}", a, b));
            }
            i64::checked_pow(*a, *b as u32).ok_or_else(|| format!("Exponent with overflow: {} ^ {}", a, b))
        }, 300);

        self.operators.insert(plus.symbol, plus);
        self.operators.insert(minus.symbol, minus);
        self.operators.insert(multiply.symbol, multiply);
        self.operators.insert(divide.symbol, divide);
        self.operators.insert(power.symbol, power);
    }

}

impl ParserConfig<f64> {
    pub fn with_basic_math(&mut self) {
        let plus = Operator::new('+', true, |a, b| Ok(a + b), 100);
        let minus = Operator::new('-', true, |a, b| Ok(a - b), 100);
        let multiply = Operator::new('*', true, |a, b| Ok(a * b), 200);
        let divide = Operator::new('/', true, |a, b| Ok(a / b), 200);
        let power = Operator::new('^', false, |a, b| {
            Ok(f64::powf(*a, *b))
        }, 300);

        self.operators.insert(plus.symbol, plus);
        self.operators.insert(minus.symbol, minus);
        self.operators.insert(multiply.symbol, multiply);
        self.operators.insert(divide.symbol, divide);
        self.operators.insert(power.symbol, power);
    }

}