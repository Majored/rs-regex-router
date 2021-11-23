// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/MetricsPage/hyper-regex-router/blob/main/LICENSE)

use crate::*;

#[test]
fn empty_builder() {
    let builder = RouterBuilder::<()>::new();
    assert!(builder.build().is_err());
}

#[test]
fn singular_string() {
    let mut builder = RouterBuilder::<&str>::new();
    route!(builder; r"^/test$";; "GET" => "Hello.");
    let router = builder.build().unwrap();

    assert!(router.dispatch("GET", "/test").is_some());
    assert!(router.dispatch("POST", "/test").is_none());
    assert!(router.dispatch("GET", "/test-test").is_none());
}