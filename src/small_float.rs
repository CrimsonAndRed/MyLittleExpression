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

    // had already parsed dot
    type Context = bool;

    fn is_operand_partial(context: &mut Self::Context, ch: char) -> bool {

        if ch.is_digit(10) {
            return true;
        }

        if ch == '.' && !*context {
            *context = true;
            return true;
        }

        false
    }

    fn from_str(from: &str) -> Result<f64, ()> {
        from.parse::<f64>().map_err(|_| {})
    }
}