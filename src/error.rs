#[derive(Debug, PartialEq, Eq)]
pub enum ExprError<'a> {
    /// Text, formula, position
    IncorrectToken(String, &'a str, usize),
    ArithmeticException(String),
    Unexpected(String),
}

impl std::fmt::Display for ExprError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ExprError::IncorrectToken(text, formula, index) => write!(f, "{}", prettify_formula_error(text, formula, *index)),
            ExprError::ArithmeticException(s) => write!(f, "Arithmetics failed at {}", &s),
            ExprError::Unexpected(s) => write!(f, "Unexpected error: {}.\n You'd better report it", &s)
        }
    }
}

impl std::error::Error for ExprError<'_> {}

fn prettify_formula_error(text: &str, formula: &str, position: usize) -> String {
    let s = "Error during formula parsing: ";

    let times_before = position;
    let times_after = if formula.len() == 0 { 0 } else { formula.len() - 1 - position };
    let mut underline: String = String::with_capacity(formula.len() + s.len());
    for _ in 0..s.len() {
        underline.push(' ');
    }
    for _ in 0..times_before {
        underline.push('_');
    }
    underline.push('^');
    for _ in 0..times_after {
        underline.push('_');
    }

    format!("{}{} <- {}\n{}", s, formula, text, &underline)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_1() {
        let s = prettify_formula_error("missing expression", "2 + * 10", 3);
        assert_eq!(s, format!("{}{}",
                               "Error during formula parsing: 2 + * 10 <- missing expression\n",
                               "                              ___^____"));
    }

    #[test]
    fn empty_formula() {
        let s = prettify_formula_error("missing formula", "", 0);
        assert_eq!(s, format!("{}{}",
                              "Error during formula parsing:  <- missing formula\n",
                              "                              ^"));
    }
}