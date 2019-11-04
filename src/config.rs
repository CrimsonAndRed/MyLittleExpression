use std::collections::HashMap;
use crate::Operator;

pub struct ParserConfig {
//    pub(crate) digit_radix: u32,
    pub(crate) operators: HashMap<char, Operator>,
}

impl Default for ParserConfig {
    fn default() -> Self {
        let mut parser = ParserConfig {
            operators: HashMap::new(),
//            digit_radix: 10u32
        };

        parser.with_basic_math();
        parser
    }
}

impl ParserConfig {
    fn with_basic_math(&mut self) {
        let plus = Operator::new('+', true, |a, b| a + b, 100);
        let minus = Operator::new('-', true, |a, b| a - b, 100);
        let multiply = Operator::new('*', true, |a, b| a * b, 200);
        let divide = Operator::new('/', true, |a, b| a / b, 200);
        let power = Operator::new('^', false, |a, b| a.pow(b as u32), 300);

        self.operators.insert(plus.symbol, plus);
        self.operators.insert(minus.symbol, minus);
        self.operators.insert(multiply.symbol, multiply);
        self.operators.insert(divide.symbol, divide);
        self.operators.insert(power.symbol, power);
    }

}

pub struct ParserConfigBuilder {
//    digit_radix: u32,
}

impl Default for ParserConfigBuilder {
    fn default() -> Self {
        ParserConfigBuilder {
//            digit_radix: 10u32
        }
    }
}
