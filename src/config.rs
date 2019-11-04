use std::collections::HashMap;
use crate::Operator;

pub struct ParserConfig<T> {
    pub(crate) operators: HashMap<char, Operator<T>>,
}

impl<T> Default for ParserConfig<T> {
    fn default() -> Self {
        let mut parser = ParserConfig {
            operators: HashMap::new(),
        };

        parser
    }
}

impl ParserConfig<i64> {
    pub fn with_basic_math(&mut self) {
        let plus = Operator::new('+', true, |a, b| a + b, 100);
        let minus = Operator::new('-', true, |a, b| a - b, 100);
        let multiply = Operator::new('*', true, |a, b| a * b, 200);
        let divide = Operator::new('/', true, |a, b| a / b, 200);
        let power = Operator::new('^', false, |a, b| i64::pow(a, b as u32), 300);

        self.operators.insert(plus.symbol, plus);
        self.operators.insert(minus.symbol, minus);
        self.operators.insert(multiply.symbol, multiply);
        self.operators.insert(divide.symbol, divide);
        self.operators.insert(power.symbol, power);
    }

}

pub struct ParserConfigBuilder {
}

impl Default for ParserConfigBuilder {
    fn default() -> Self {
        ParserConfigBuilder {
        }
    }
}
