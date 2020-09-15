# Simple Expression Solver

This small project is capable of solving simple formulas!
It may be used to solve expressions in different notaitons, even not usual ones (see [this](tests/unary_numeric_system.rs) example).

## Usage
Add the following line to Cargo.toml file:
```
expression_parser = { git = "https://github.com/CrimsonAndRed/MyLittleExpression" }
```

To calculate an expression follow the code below:

```rust
use expression_parser::config::ParserConfig;

fn main() {
    // Define ParserConfig for given numeric type.
    let mut pc = ParserConfig::<i64>::default();
    // Add operators to ParserConfig.
    pc.with_basic_math();
    // Parse the expression.
    let result = pc.parse("1 + 1");
    println!("{:?}", result);
}
```

## TODO
- variables as strings
- comments