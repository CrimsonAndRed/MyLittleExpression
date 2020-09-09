extern crate expression_parser;

pub mod setup;

#[test]
fn basic_add() {
    let num = setup::setup_basic_config().parse(&"1 + 2");
    assert_eq!(Ok(3i64), num);
}

#[test]
fn basic_sub() {
    let num = setup::setup_basic_config().parse(&"2 - 1");
    assert_eq!(Ok(1i64), num);
}

#[test]
fn basic_mul() {
    let num = setup::setup_basic_config().parse(&"3 * 2");
    assert_eq!(Ok(6i64), num);
}

#[test]
fn basic_div() {
    let num = setup::setup_basic_config().parse(&"4 / 2");
    assert_eq!(Ok(2i64), num);
}

#[test]
fn basic_multidigits() {
    let num = setup::setup_basic_config().parse(&"100 + 200");
    assert_eq!(Ok(300i64), num);
}

#[test]
fn basic_several() {
    let num = setup::setup_basic_config().parse(&"2 + 8 - 5");
    assert_eq!(Ok(5i64), num);
}

#[test]
fn basic_nospace() {
    let num = setup::setup_basic_config().parse(&"2+8-5");
    assert_eq!(Ok(5i64), num);
}

#[test]
fn basic_power() {
    let num = setup::setup_basic_config().parse(&"2^5");
    assert_eq!(Ok(32i64), num);
}


#[test]
fn basic_power_2() {
    let num = setup::setup_basic_config().parse(&"2^0");
    assert_eq!(Ok(1i64), num);
}

#[test]
fn basic_power_3() {
    let num = setup::setup_basic_config().parse(&"2^10");
    assert_eq!(Ok(1024i64), num);
}

#[test]
fn basic_single_num() {
    let num = setup::setup_basic_config().parse(&"20");
    assert_eq!(Ok(20i64), num);
}

