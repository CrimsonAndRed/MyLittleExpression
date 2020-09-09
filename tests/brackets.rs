extern crate expression_parser;

pub mod setup;

#[test]
fn brackets_1() {
    let num = setup::setup_basic_config().parse(&"(1 + 2) * 2");
    assert_eq!(Ok(6i64), num);
}

#[test]
fn brackets_2() {
    let num = setup::setup_basic_config().parse(&"((1 + 2) * 2)");
    assert_eq!(Ok(6i64), num);
}

#[test]
fn brackets_3() {
    let num = setup::setup_basic_config().parse(&"(4 /2) * (2 + 1)");
    assert_eq!(Ok(6i64), num);
}

#[test]
fn brackets_4() {
    let num = setup::setup_basic_config().parse(&"((4 /2) * (2 + 1))");
    assert_eq!(Ok(6i64), num);
}

#[test]
fn brackets_5() {
    let num = setup::setup_basic_config().parse(&"(((4/2) * (2 + 1)) - 1)/5");
    assert_eq!(Ok(1i64), num);
}
