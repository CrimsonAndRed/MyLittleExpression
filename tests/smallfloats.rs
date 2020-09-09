extern crate expression_parser;

pub mod setup;

#[test]
fn basic_add() {
    let num = setup::setup_floats_config().parse(&"1 + 2");
    assert_eq!(Ok(3.0f64), num);
}

#[test]
fn basic_sub() {
    let num = setup::setup_floats_config().parse(&"2.3 - 2.3");
    assert_eq!(Ok(0f64), num);
}

#[test]
fn basic_mul() {
    let num = setup::setup_floats_config().parse(&"2.0 * 1.5");
    assert_eq!(Ok(3.0f64), num);
}

#[test]
fn basic_floatshit_1() {
    let num = setup::setup_floats_config().parse(&"0.2 + 0.1");
    assert_eq!(Ok(0.30000000000000004f64), num);
}

#[test]
fn basic_floatshit_2() {
    let num = setup::setup_floats_config().parse(&"3.2 - 3.1");
    assert_eq!(Ok(0.10000000000000009f64), num);
}

#[test]
fn basic_div() {
    let num = setup::setup_floats_config().parse(&"10 / 2.5");
    assert_eq!(Ok(4.0f64), num);
}