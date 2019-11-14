use std::collections::VecDeque;
use crate::config::ParserConfig;
use crate::Operator;
use crate::Operand;
use crate::ExprError;

impl<'a, T> ParserConfig<T> where T: Operand {

    fn parse_operator(&self, from: &[char]) -> Option<(usize, &Operator<T>)> {
        self.operators.get(&from[0]).map(|i| (1usize, i))
    }

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
                }
            }
        }

        let res = rpn.pop_back().ok_or_else(|| ExprError::Unexpected("Could not get solution because RPN stack was too short".to_owned()))?;
        if !rpn.is_empty() {
            return Err(ExprError::Unexpected(format!("Processed until solution but there was {} more items in RPN queue", rpn.len())));
        }
        Ok(res)
    }


    pub(crate) fn yard_from_str(&self, formula: &'a str) -> Result<VecDeque<RPNToken<T>>, ExprError<'a>> {
        let mut yard = VecDeque::new();
        let mut operators: VecDeque<YardToken<T>> = VecDeque::new();

        // No support for surrogate pairs
        let mut index = 0usize;
        let chars_origin = &formula.chars().collect::<Vec<char>>()[..];
        let bound = chars_origin.len();

        while index < bound {
            let chars = &chars_origin[index..];
            // Predefined most-priority symbols: whitespaces, open and close brackets
            match chars[0] {
                x if x.is_whitespace() => {
                    index += 1;
                    continue;
                }
                '(' => {
                    operators.push_back(YardToken::OpenBracket(index));
                    index += 1;
                    continue;
                }
                ')' => {
                    loop {
                        let last_operator = operators.pop_back().ok_or_else(|| ExprError::IncorrectToken("Close bracket does not have corresponing open bracket".to_owned(), formula, index))?;

                        match last_operator {
                            YardToken::OpenBracket(_) => break,
                            YardToken::Operator(op, _) => yard.push_back(RPNToken::Operator(op)),
                        }
                    }
                    index += 1;
                    continue;
                }
                _ => {}
            }

            let parsed_operand = T::parse_operand(&chars);
            let parsed_operator = self.parse_operator(&chars);

            // Should have match expression here, but it is hard
            if parsed_operand.is_some() &&
                (parsed_operator.is_none() || parsed_operator.as_ref().unwrap().0 > parsed_operator.as_ref().unwrap().0) {
                let (i, operand) = parsed_operand.unwrap();
                yard.push_back(RPNToken::Operand(operand));
                index += i;
                continue;
            }

            if parsed_operator.is_some() &&
                (parsed_operand.is_none() || parsed_operator.as_ref().unwrap().0 > parsed_operand.as_ref().unwrap().0) {

                let (j, operator) = parsed_operator.unwrap();
                loop {
                    let last_operator = operators.back();
                    if last_operator.is_none() {
                        break;
                    }
                    // Safe unwrap
                    if let YardToken::Operator(last_operator, _) = last_operator.unwrap() {
                        if last_operator.order > operator.order
                            ||
                            last_operator.left_associative && last_operator.order == operator.order {
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
                operators.push_back(YardToken::Operator(operator, j));
                index += j;
                continue;
            }


            if parsed_operator.is_some() && parsed_operand.is_some() {
                return Err(ExprError::IncorrectToken(format!("Could not determine if following expression is operator {} or operand {}", parsed_operator.unwrap().1.symbol, parsed_operand.unwrap().1), formula, index));
            } else {
                return Err(ExprError::IncorrectToken("No matching operand or operator".to_owned(), formula, index));
            }
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