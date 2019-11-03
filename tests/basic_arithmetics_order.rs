extern crate expression_parser;

mod setup;

#[test]
fn basic_order_1() {
    let num = setup::setup_basic_config().parse(&"1 + 2 * 2");
    assert_eq!(5i64, num);
}

#[test]
fn basic_order_2() {
    let num = setup::setup_basic_config().parse(&"1 + 2 / 2");
    assert_eq!(2i64, num);
}

#[test]
fn basic_order_3() {
    let num = setup::setup_basic_config().parse(&"1 + 2 * 2 - 3 * 10");
    assert_eq!(-25i64, num);
}

#[test]
fn basic_order_4() {
    let num = setup::setup_basic_config().parse(&"12*2-32/8");
    assert_eq!(20i64, num);
}

#[test]
fn basic_order_5() {
    let num = setup::setup_basic_config().parse(&"6/2*3+1");
    assert_eq!(10i64, num);
}