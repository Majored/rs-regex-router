// Copyright (c) 2021 MetricsPage [Harry Plumb]
// MIT License (https://github.com/MetricsPage/hyper-regex-router/blob/main/LICENSE)

use crate::*;

#[test]
fn empty_builder() {
    let mut builder = RouterBuilder::<String>::new();
    assert!(builder.build().is_err());
}