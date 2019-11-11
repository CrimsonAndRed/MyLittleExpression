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
        let plus = Operator::new('+', true, |a, b| Ok(a + b), 100);
        let minus = Operator::new('-', true, |a, b| Ok(a - b), 100);
        let multiply = Operator::new('*', true, |a, b| Ok(a * b), 200);
        let divide = Operator::new('/', true, |a, b| Ok(a / b), 200);
        let power = Operator::new('^', false, |a, b| {
            // TODO https://docs.rs/num-traits/0.2.5/src/num_traits/cast.rs.html#197-204
            Ok(i64::pow(*a, *b as u32))
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
            // TODO https://docs.rs/num-traits/0.2.5/src/num_traits/cast.rs.html#197-204
            Ok(f64::powf(*a, *b))
        }, 300);

        self.operators.insert(plus.symbol, plus);
        self.operators.insert(minus.symbol, minus);
        self.operators.insert(multiply.symbol, multiply);
        self.operators.insert(divide.symbol, divide);
        self.operators.insert(power.symbol, power);
    }

}