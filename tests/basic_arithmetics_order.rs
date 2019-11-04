extern crate expression_parser;

mod setup;

#[test]
fn basic_order_1() {
    let num = setup::setup_basic_config().parse(&"1 + 2 * 2");
    assert_eq!(Ok(5i64), num);
}

#[test]
fn basic_order_2() {
    let num = setup::setup_basic_config().parse(&"1 + 2 / 2");
    assert_eq!(Ok(2i64), num);
}

#[test]
fn basic_order_3() {
    let num = setup::setup_basic_config().parse(&"1 + 2 * 2 - 3 * 10");
    assert_eq!(Ok(-25i64), num);
}

#[test]
fn basic_order_4() {
    let num = setup::setup_basic_config().parse(&"12*2-32/8");
    assert_eq!(Ok(20i64), num);
}

#[test]
fn basic_order_5() {
    let num = setup::setup_basic_config().parse(&"6/2*3+1");
    assert_eq!(Ok(10i64), num);
}

#[test]
fn power_order_1() {
    let num = setup::setup_basic_config().parse(&"6/2*3^2");
    assert_eq!(Ok(27i64), num);
}

#[test]
fn power_order_2() {
    let num = setup::setup_basic_config().parse(&"(3-2)^(2*2)");
    assert_eq!(Ok(1i64), num);
}

#[test]
fn power_order_3() {
    let num = setup::setup_basic_config().parse(&"2*2^10/2");
    assert_eq!(Ok(1024i64), num);
}
