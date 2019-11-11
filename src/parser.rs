use std::collections::VecDeque;
use crate::config::ParserConfig;
use crate::Operator;
use crate::Operand;
use crate::ExprError;


impl<'a, T> ParserConfig<T> where T: Operand {
    pub fn parse(&self, formula: &'a str) -> Result<T, ExprError<'a>> {

        let mut yard = self.yard_from_str(formula)?;

        let mut rpn: VecDeque<T> = VecDeque::new();

        for token in yard.drain(..) {
            match token {
                RPNToken::Operand(num) => rpn.push_back(num),
                RPNToken::Operator(op) => {
                    let arg2 = rpn.pop_back().ok_or_else(|| ExprError::Unexpected(format!("Not enough arguments for operator {}", op.symbol)))?;
                    let arg1 = rpn.pop_back().ok_or_else(|| ExprError::Unexpected(format!("Not enough arguments for operator {}", op.symbol)))?;
                    // Pass arguments by reference - is it ok?
                    rpn.push_back(
                        (op.operation)(&arg1, &arg2).map_err(|e| ExprError::ArithmeticException(format!("Error during computation {} over arguments {} and {}:\n{}", &op.symbol, &arg1, &arg2, &e)))?
                    );
                },
            }
        }

        let res = rpn.pop_back().ok_or_else(|| ExprError::Unexpected("Could not get solution because RPN stack was too short".to_owned()))?;
        if !rpn.is_empty() {
            return Err(ExprError::Unexpected(format!("Processed until solution but there was {} more items in RPN queue", rpn.len())));
        }
        Ok(res)
    }
}

impl<'a, T> ParserConfig<T> where T: Operand {
    pub(crate) fn yard_from_str(&self, formula: &'a str) -> Result<VecDeque<RPNToken<T>>, ExprError<'a>> {
        let mut yard = VecDeque::new();
        let mut operators: VecDeque<YardToken<T>> = VecDeque::new();

        let mut num = String::new();
        let mut context = T::Context::default();

        let mut iter = formula.chars().enumerate();
        while let Some((i, ch)) = iter.next() {

            if T::is_operand_partial(&mut context, ch) {
                num.push(ch);
                // TODO operand and operator collision here
                continue;
            } else if !num.is_empty() {
                let n = T::from_str(&num).map_err(|_| ExprError::IncorrectToken("Failed to parse number at index".to_owned(), formula, i))?;
                yard.push_back(RPNToken::Operand(n));
                num = String::new();
                context = T::Context::default();
            }

            if ch.is_whitespace() {
                continue;
            }

            if ch == '(' {
                operators.push_back(YardToken::OpenBracket(i));
                continue;
            }

            if ch == ')' {
                loop {
                    let last_operator = operators.pop_back().ok_or_else(|| ExprError::IncorrectToken("Close bracket does not have corresponing open bracket".to_owned(), formula, i))?;

                    match last_operator {
                        YardToken::OpenBracket(_) => break,
                        YardToken::Operator(op, _) => yard.push_back(RPNToken::Operator(op)),
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
                if let YardToken::Operator(last_operator, _) = last_operator.unwrap() {
                    if last_operator.order > curr_operator.order
                        ||
                        last_operator.left_associative && last_operator.order == curr_operator.order {
                        // And safe unwrap here again
                        if let YardToken::Operator(op, _) = operators.pop_back().unwrap() {
                            yard.push_back(RPNToken::Operator(op));
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            operators.push_back(YardToken::Operator(curr_operator, i));
        }

        if !num.is_empty() {
            let n = T::from_str(&num).map_err(|_| ExprError::IncorrectToken(format!("Could not parse number {} ", &num), formula, formula.len() - num.len()))?;
            yard.push_back(RPNToken::Operand(n));
        }

        for op in operators.iter().rev() {
            let op = match op {
                YardToken::Operator(op, _) => *op,
                YardToken::OpenBracket(index) => return Err(ExprError::IncorrectToken("Open bracket is not closed".to_owned(), formula, *index))
            };

            yard.push_back(RPNToken::Operator(op))
        };

        Ok(yard)
    }
}

pub(crate) enum RPNToken<'a, T> where T: Operand {
    Operand(T),
    Operator(&'a Operator<T>),
}

pub(crate) enum YardToken<'a, T> where T: Operand {
    OpenBracket(usize),
    Operator(&'a Operator<T>, usize),
}