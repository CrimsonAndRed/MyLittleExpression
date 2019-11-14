use crate::Operator;
use crate::Operand;
use crate::ParserConfig;

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

impl Operand for f64 {
    fn parse_operand(from: &[char]) -> Option<(usize, Self)> {
        let mut index = 0usize;
        let mut has_dot = false;

        while index < from.len() {
            match from[index] {
                ch if ch.is_digit(10) => {
                    index += 1;
                    continue;
                },
                '.' if has_dot => {
                    break
                },
                '.' if !has_dot => {
                    has_dot = true;
                    index+=1;
                    continue;
                },
                _ => break
            }
        }

        if index == 0 {
            return None;
        }


        let parsed = from[..index].iter().collect::<String>().parse::<f64>();
        parsed.ok().map(|p| (index, p))
    }
}