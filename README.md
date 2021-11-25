# rs-regex-router
[![GitHub license](https://img.shields.io/badge/license-MIT-007ec6)](https://github.com/MetricsPage/fast-regex-router/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/regex_router)](https://crates.io/crates/regex_router)
[![docs.rs](https://img.shields.io/docsrs/regex_router)](https://docs.rs/regex_router/)

An implementation of request routing via a singular grouped regex (with support for path parameter extraction).

## Features
- Design approach based upon [this](https://www.npopov.com/2014/02/18/Fast-request-routing-using-regular-expressions.html) article.
- Implemented over a generic handler type allowing use with varying webserver crates.
- Extraction of path parameters which are then mapped as key/value pairs.
- A convenience macro for easily declaring routes.

## Installation & Basic Usage
```toml
[dependencies]
regex_router = "1.0.0"
```

An example using a unit handler type:
```rust
use regex_router::{RouterBuilder, route};
...

let mut builder = RouterBuilder::<()>::new();

route!(builder; r"/example";; "GET" => ());
route!(builder; r"/test";; "GET" => ());

let router = builder.build().unwrap();
```

An example declaring path parameters:
```rust
let mut builder = RouterBuilder::<()>::new();

route!(builder; r"/example/(\d+)"; "var1"; "GET" => ());
route!(builder; r"/test/(\d+)/(\d+)"; "var1", "var2"; "GET" => ());

let router = builder.build().unwrap();
```

Dispatching against a router:
```rust
match router.dispatch("GET", "/example/500") {
  Some(route_match) => {
    // Call handler and return response.
  }
  None => {
    // No route match. Return 404.
  }
};
```

An example implementation for `hyper` can be found [here](https://github.com/Majored/rs-regex-router/blob/main/examples/hyper_sync.rs).

## Issues & Support
Whether you're wanting to report a bug you've come across during use of this crate or are seeking general help/assistance, please utilise the [issues tracker](https://github.com/Majored/rs-regex-router/issues) and provide as much detail as possible (eg. recreation steps).

I try to respond to issues within a reasonable timeframe.
