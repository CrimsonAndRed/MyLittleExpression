use std::collections::HashMap;
use crate::Operator;
use crate::Operand;
use crate::Variable;

pub struct ParserConfig<T> where T: Operand {
    pub(crate) operators: HashMap<char, Operator<T>>,
    pub(crate) variables: HashMap<char, Variable<T>>,
}

impl<T> ParserConfig<T> where T: Operand {
    pub fn with_operator(&mut self, op: Operator<T>) {
        self.operators.insert(op.symbol, op);
    }

    pub fn with_variable(&mut self, op: Variable<T>) {
        self.variables.insert(op.symbol, op);
    }
}

impl<T> Default for ParserConfig<T> where T: Operand {
    fn default() -> Self {
        let parser = ParserConfig {
            operators: HashMap::new(),
            variables: HashMap::new(),
        };

        parser
    }
}