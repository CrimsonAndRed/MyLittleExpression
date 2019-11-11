use expression_parser::config::ParserConfig;

#[cfg(test)]
pub(crate) fn setup_basic_config() -> ParserConfig<i64> {
    let mut cfg = ParserConfig::<i64>::default();
    cfg.with_basic_math();
    cfg
}

#[cfg(test)]
pub(crate) fn setup_floats_config() -> ParserConfig<f64> {
    let mut cfg = ParserConfig::<f64>::default();
    cfg.with_basic_math();
    cfg
}
