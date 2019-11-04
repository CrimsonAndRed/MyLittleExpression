use std::collections::VecDeque;
use crate::config::ParserConfig;
use crate::Operator;
use crate::ExprError;

impl ParserConfig {
    pub(crate) fn yard_from_str(&self, formula: &str) -> Result<VecDeque<RPNToken>, ExprError> {
        let mut yard = VecDeque::new();
        let mut operators: VecDeque<YardToken> = VecDeque::new();

        let mut num = String::new();
        let mut state = ParserState::NotInNumber;

        let mut iter = formula.chars().enumerate();
        while let Some((i, ch)) = iter.next() {
            if ch.is_digit(10) {
                num.push(ch);
                state = ParserState::InNumber;
                continue;
            } else if state == ParserState::InNumber {

                let n = i64::from_str_radix(&num, 10).map_err(|_| ExprError::IncorrectToken(format!("Failed to parse number at index {}", i)))?;
                yard.push_back(RPNToken::Number(n));
                num = String::new();
            }

            if ch.is_whitespace() {
                state = ParserState::NotInNumber;
                continue;
            }

            if ch == '(' {
                state = ParserState::NotInNumber;
                operators.push_back(YardToken::OpenBracket);
                continue;
            }

            if ch == ')' {
                state = ParserState::NotInNumber;
                loop {
                    let last_operator = operators.pop_back().ok_or_else(|| ExprError::IncorrectToken(format!("Close bracket at index {} does not have corresponing open bracket", i)))?;

                    match last_operator {
                        YardToken::OpenBracket => break,
                        YardToken::Operator(op) => yard.push_back(RPNToken::Operator(op)),
                    }
                }
                continue;
            }

            let curr_operator = self.operators.get(&ch).ok_or_else(|| ExprError::IncorrectToken(format!("Missed operator symbol {}", &ch)))?;
            loop {

                let last_operator = operators.back();
                if last_operator.is_none() {
                    break;
                }
                // Safe unwrap
                if let YardToken::Operator(ref last_operator) = last_operator.unwrap() {
                    if last_operator.order > curr_operator.order
                        ||
                        last_operator.left_associative && last_operator.order == curr_operator.order {
                        // And safe unwrap here again
                        if let YardToken::Operator(op) = operators.pop_back().unwrap() {
                            yard.push_back(RPNToken::Operator(op));
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            operators.push_back(YardToken::Operator(curr_operator));
        }

        if !num.is_empty() {
            let n = i64::from_str_radix(&num, 10).map_err(|_| ExprError::IncorrectToken(format!("Could not parse number {} ", &num)))?;

            yard.push_back(RPNToken::Number(n));
        }

        operators.iter().rev().for_each(|op| {
            let op = match op {
                YardToken::Operator(op) => op,
                _ => panic!("Unwind non operator from operators stack"),
            };

            yard.push_back(RPNToken::Operator(*op))
        });

        Ok(yard)
    }
}

#[derive(Debug)]
pub(crate) enum RPNToken<'a> {
    Number(i64),
    Operator(&'a Operator),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ParserState {
    InNumber,
    NotInNumber,
}

pub(crate) enum YardToken<'a> {
    OpenBracket,
    Operator(&'a Operator),
}
