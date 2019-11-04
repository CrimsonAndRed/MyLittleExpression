extern crate expression_parser;

mod setup;

#[test]
fn basic_add() {
    let num = setup::setup_basic_config().parse(&"1 + 2");
    assert_eq!(3i64, num);
}

#[test]
fn basic_sub() {
    let num = setup::setup_basic_config().parse(&"2 - 1");
    assert_eq!(1i64, num);
}

#[test]
fn basic_mul() {
    let num = setup::setup_basic_config().parse(&"3 * 2");
    assert_eq!(6i64, num);
}

#[test]
fn basic_div() {
    let num = setup::setup_basic_config().parse(&"4 / 2");
    assert_eq!(2i64, num);
}

#[test]
fn basic_multidigits() {
    let num = setup::setup_basic_config().parse(&"100 + 200");
    assert_eq!(300i64, num);
}

#[test]
fn basic_several() {
    let num = setup::setup_basic_config().parse(&"2 + 8 - 5");
    assert_eq!(5i64, num);
}

#[test]
fn basic_nospace() {
    let num = setup::setup_basic_config().parse(&"2+8-5");
    assert_eq!(5i64, num);
}

#[test]
fn basic_power() {
    let num = setup::setup_basic_config().parse(&"2^5");
    assert_eq!(32i64, num);
}


#[test]
fn basic_power_2() {
    let num = setup::setup_basic_config().parse(&"2^0");
    assert_eq!(1i64, num);
}

#[test]
fn basic_power_3() {
    let num = setup::setup_basic_config().parse(&"2^10");
    assert_eq!(1024i64, num);
}
