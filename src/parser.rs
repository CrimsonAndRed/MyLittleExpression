use std::collections::VecDeque;
use crate::config::ParserConfig;
use crate::Operator;
use crate::ExprError;

impl <'a> ParserConfig<i64> {
    pub(crate) fn  yard_from_str(&self, formula: &'a str) -> Result<VecDeque<RPNToken<i64>>, ExprError<'a>> {
        let mut yard = VecDeque::new();
        let mut operators: VecDeque<YardToken<i64>> = VecDeque::new();

        let mut num = String::new();
        let mut state = ParserState::NotInNumber;

        let mut iter = formula.chars().enumerate();
        while let Some((i, ch)) = iter.next() {
            if ch.is_digit(10) {
                num.push(ch);
                state = ParserState::InNumber;
                continue;
            } else if state == ParserState::InNumber {

                let n = i64::from_str_radix(&num, 10).map_err(|_| ExprError::IncorrectToken("Failed to parse number at index".to_owned(), formula, i))?;
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
                    let last_operator = operators.pop_back().ok_or_else(|| ExprError::IncorrectToken("Close bracket does not have corresponing open bracket".to_owned(), formula, i))?;

                    match last_operator {
                        YardToken::OpenBracket => break,
                        YardToken::Operator(op) => yard.push_back(RPNToken::Operator(op)),
                    }
                }
                continue;
            }

            let curr_operator = self.operators.get(&ch).ok_or_else(|| ExprError::IncorrectToken(format!("Missed operator symbol {}", &ch), formula, i))?;
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
            // TODO position in formula
            let n = i64::from_str_radix(&num, 10).map_err(|_| ExprError::IncorrectToken(format!("Could not parse number {} ", &num), formula, formula.len() - 1))?;

            yard.push_back(RPNToken::Number(n));
        }

        for op in operators.iter().rev() {
            let op = match op {
                YardToken::Operator(op) => op,
                // TODO position in formula
                _ => return Err(ExprError::IncorrectToken("Open bracket is not closed".to_owned(), formula, formula.len() - 1))
                
            };

            yard.push_back(RPNToken::Operator(*op))
        };

        Ok(yard)
    }
}

#[derive(Debug)]
pub(crate) enum RPNToken<'a, T> {
    Number(i64),
    Operator(&'a Operator<T>),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ParserState {
    InNumber,
    NotInNumber,
}

pub(crate) enum YardToken<'a, T> {
    OpenBracket,
    Operator(&'a Operator<T>),
}
