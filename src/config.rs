use std::collections::HashMap;
use crate::Operator;
use crate::Operand;

pub struct ParserConfig<T> where T: Operand {
    pub(crate) operators: HashMap<char, Operator<T>>,
}

impl<T> ParserConfig<T> where T: Operand {
    pub fn with_operator(&mut self, op: Operator<T>) {
        self.operators.insert(op.symbol, op);
    }
}

impl<T> Default for ParserConfig<T> where T: Operand {
    fn default() -> Self {
        let parser = ParserConfig {
            operators: HashMap::new(),
        };

        parser
    }
}