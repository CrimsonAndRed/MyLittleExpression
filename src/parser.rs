use std::collections::VecDeque;
use crate::config::ParserConfig;
use crate::Operator;

impl ParserConfig {

    pub(crate) fn yard_from_str(&self, formula: &str) -> VecDeque<YardToken> {
        let mut yard = VecDeque::new();
        let mut operators: VecDeque<&Operator> = VecDeque::new();

        let mut num = String::new();
        let mut state = ParserState::NotInNumber;

        let mut iter = formula.chars().into_iter();
        while let Some(ch) = iter.next() {
            if ch.is_digit(10) {
                num.push(ch);
                state = ParserState::InNumber;
                continue;
            } else if state == ParserState::InNumber {
                yard.push_back(YardToken::Number(i64::from_str_radix(&num, 10).unwrap()));
                num = String::new();
            }

            if ch.is_whitespace() {
                state = ParserState::NotInNumber;
                continue;
            }

            let curr_operator = self.operators.get(&ch).unwrap();
            loop {
                let last_operator = operators.back();
                if last_operator.is_none() {
                    break;
                }

                let last_operator = *last_operator.unwrap();

                if last_operator.order > curr_operator.order
                    ||
                    last_operator.left_associative && last_operator.order == curr_operator.order {
                    yard.push_back(YardToken::Operator(operators.pop_back().unwrap()));
                } else {
                    break;
                }
            }
            operators.push_back(curr_operator);
        }

        if !num.is_empty() {
            yard.push_back(YardToken::Number(i64::from_str_radix(&num, 10).unwrap()));
        }

        operators.iter().rev().for_each(|op| yard.push_back(YardToken::Operator(*op)));

        yard
    }
}

#[derive(Debug)]
pub(crate) enum YardToken<'a> {
    Number(i64),
    Operator(&'a Operator)
}

#[derive(Debug, PartialEq)]
pub(crate) enum ParserState {
    InNumber,
    NotInNumber,
}