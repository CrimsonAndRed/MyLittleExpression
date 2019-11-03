pub mod config;

mod parser;

use std::collections::VecDeque;

use config::ParserConfig;
use parser::yard_from_str;
use parser::YardToken;

impl ParserConfig {
    pub fn parse(&self, formula: &str) -> i64 {

        let mut yard = yard_from_str(formula);

        let mut rpn: VecDeque<i64> = VecDeque::new();

        for token in yard.iter() {
            println!("{:?}", token);
            match token {
                YardToken::Number(num) => rpn.push_back(i64::from_str_radix(&num, 10).unwrap()),
                YardToken::Operator(op) => {
                    let arg2 =  rpn.pop_back().unwrap();
                    let arg1 = rpn.pop_back().unwrap();
                    rpn.push_back(
                        self.applyBinaryOperator(op.chars().next().unwrap(), arg1, arg2)
                    );
                },
            }
        }

        let res = rpn.pop_back().unwrap();
        assert!(rpn.is_empty());
        res
    }

    fn applyBinaryOperator(&self, op: char, arg1: i64, arg2: i64) -> i64 {
        match op {
            '+' => arg1 + arg2,
            '-' => arg1 - arg2,
            '*' => arg1 * arg2,
            '/' => arg1 / arg2,
            _ => unimplemented!("Not yet")
        }
    }
}