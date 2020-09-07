extern crate expression_parser;

mod setup;

#[cfg(feature = "bigint")]
use num_bigint::BigInt;

#[cfg(feature = "bigint")]
use std::str::FromStr;

#[test]
#[cfg(feature = "bigint")]
fn basic_add() {
    let num = setup::setup_big_int_config().parse(&"10 + 20");
    assert_eq!(Ok(BigInt::from_str("30").unwrap()), num);
}


#[test]
#[cfg(feature = "bigint")]
fn basic_sub() {
    let num = setup::setup_big_int_config().parse(&"25 - 15");
    assert_eq!(Ok(BigInt::from_str("10").unwrap()), num);
}

#[test]
#[cfg(feature = "bigint")]
fn basic_mul() {
    let num = setup::setup_big_int_config().parse(&"35 * 2");
    assert_eq!(Ok(BigInt::from_str("70").unwrap()), num);
}

#[test]
#[cfg(feature = "bigint")]
fn basic_div() {
    let num = setup::setup_big_int_config().parse(&"4 / 2");
    assert_eq!(Ok(BigInt::from_str("2").unwrap()), num);
}

#[test]
#[cfg(feature = "bigint")]
fn overflow_i64_add() {
    let num = setup::setup_big_int_config().parse(&"9223372036854775807 * 2");
    assert_eq!(Ok(BigInt::from_str("18446744073709551614").unwrap()), num);
}
