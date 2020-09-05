extern crate expression_parser;

mod setup;
use expression_parser::Variable;

#[test]
fn basic_add() {
    let mut cfg = setup::setup_basic_config();
    cfg.with_variable(Variable::new('a', 100i64));
    let num = cfg.parse(&"a + 1");
    assert_eq!(Ok(101i64), num);
}

