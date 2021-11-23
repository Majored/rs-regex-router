# fast-regex-router
[![GitHub license](https://img.shields.io/badge/license-MIT-007ec6)](https://github.com/MetricsPage/fast-regex-router/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/regex_router)](https://crates.io/crates/regex_router)
[![docs.rs](https://img.shields.io/docsrs/regex_router)](https://docs.rs/regex_router/)

An implementation of request routing via a singular grouped regex (with support for path parameter extraction).

## Features
- Implemented based upon [this](https://www.npopov.com/2014/02/18/Fast-request-routing-using-regular-expressions.html) article.
- Implemented over a generic handler type allowing use with varying webserver crates.
- Extraction of path parameters which are then mapped as key/value pairs.
- Convenience macros for easily declaring routes and obtaining path parameter values.
- Etc...

## Installation & Basic Usage
```toml
[dependencies]
fast_regex_router = { git="https://github.com/MetricsPage/fast-regex-router/" }
```

```Rust
let mut builder = RouterBuilder::<String>::new();
let router = builder.build().unwrap();
```
