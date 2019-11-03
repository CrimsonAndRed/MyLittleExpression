use std::collections::VecDeque;

type ShuntingYard = VecDeque<YardToken>;

pub(crate) fn yard_from_str(formula: &str) -> ShuntingYard {

    let mut yard = ShuntingYard::new();
    let mut operands: VecDeque<char> = VecDeque::new();

    let mut num = String::new();
    let mut state = ParserState::NotInNumber;

    let mut iter = formula.chars().into_iter();
    while let Some(ch) = iter.next() {
        if ch.is_digit(10) {
            num.push(ch);
            state = ParserState::InNumber;
            continue;
        } else if state == ParserState::InNumber {
            yard.push_back(YardToken::Number(num));
            num = String::new();
        }

        if ch.is_whitespace() {
            state = ParserState::NotInNumber;
            continue;
        }

        operands.drain(0..).for_each(|op| yard.push_back(YardToken::Operator(op.to_string())));
        operands.push_back(ch);
    }

    if !num.is_empty() {
        yard.push_back(YardToken::Number(num));
    }

    operands.iter().for_each(|op| yard.push_back(YardToken::Operator(op.to_string())));

    yard
}

#[derive(Debug, PartialEq)]
pub(crate) enum YardToken {
    Number(String),
    Operator(String)
}

#[derive(Debug, PartialEq)]
pub(crate) enum ParserState {
    InNumber,
    NotInNumber,
}