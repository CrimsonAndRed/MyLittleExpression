use expression_parser::config::ParserConfig;

#[cfg(test)]
pub fn setup_basic_config() -> ParserConfig<i64> {
    let mut cfg = ParserConfig::<i64>::default();
    cfg.with_basic_math();
    cfg
}
