extern crate expression_parser;

mod setup;

#[test]
fn basic_negative_1() {
    let num = setup::setup_basic_config().parse(&"");
    assert!(num.is_err());
}

#[test]
fn basic_negative_3() {
    let num = setup::setup_basic_config().parse(&"2 - ");
    assert!(num.is_err());
}

#[test]
fn basic_negative_4() {
    let num = setup::setup_basic_config().parse(&"2 + 1`");
    assert!(num.is_err());
}

#[test]
fn basic_negative_5() {
    let num = setup::setup_basic_config().parse(&")");
    assert!(num.is_err());
}

#[test]
fn basic_negative_6() {
    let num = setup::setup_basic_config().parse(&"()");
    assert!(num.is_err());
}

#[test]
fn basic_negative_7() {
    let num = setup::setup_basic_config().parse(&"(29 - 3");
    assert!(num.is_err());
}

#[test]
fn basic_negative_8() {
    let num = setup::setup_basic_config().parse(&"(29 - 3))");
    assert!(num.is_err());
}

#[test]
fn basic_negative_9() {
    let num = setup::setup_basic_config().parse(&"10 10");
    assert!(num.is_err());
}

#[test]
fn basic_negative_10() {
    let num = setup::setup_basic_config().parse(&"80 + * 2");
    assert!(num.is_err());
}