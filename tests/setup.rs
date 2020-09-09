use expression_parser::config::ParserConfig;

pub fn setup_basic_config() -> ParserConfig<i64> {
    let mut cfg = ParserConfig::<i64>::default();
    cfg.with_basic_math();
    cfg
}


pub fn setup_floats_config() -> ParserConfig<f64> {
    let mut cfg = ParserConfig::<f64>::default();
    cfg.with_basic_math();
    cfg
}

#[cfg(feature = "bigint")]
use num_bigint::BigInt;
#[cfg(feature = "bigint")]
pub fn setup_big_int_config() -> ParserConfig<BigInt> {
    let mut cfg = ParserConfig::<BigInt>::default();
    cfg.with_basic_math();
    cfg
}