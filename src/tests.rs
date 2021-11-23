// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/MetricsPage/hyper-regex-router/blob/main/LICENSE)

use crate::*;

#[test]
fn empty() {
    let builder = RouterBuilder::<()>::new();
    assert!(builder.build().is_err());
}

#[test]
fn singular() {
    let mut builder = RouterBuilder::<&str>::new();
    route!(builder; r"^/test$";; "GET" => "Hello.");
    let router = builder.build().unwrap();

    assert_eq!(*router.dispatch("GET", "/test").unwrap().handler(), "Hello.");
    assert!(router.dispatch("POST", "/test").is_none());
    assert!(router.dispatch("GET", "/test-test").is_none());
}

#[test]
fn singular_variable() {
    let mut builder = RouterBuilder::<&str>::new();
    route!(builder; r"^/test/(.+)$"; "example"; "GET" => "Hello.");
    let router = builder.build().unwrap();

    let r_match = router.dispatch("GET", "/test/World.").unwrap();
    let param = r_match.params().get("example").unwrap();

    assert_eq!(*param, "World.");
    assert_eq!(*r_match.handler(), "Hello.");

    assert!(router.dispatch("POST", "/test/World.").is_none());
    assert!(router.dispatch("GET", "/test-test").is_none());
}

#[test]
fn multiple_methods() {
    let mut builder = RouterBuilder::<&str>::new();
    route!(builder; r"^/test$";; "GET" => "Hello.", "POST" => "World.");
    let router = builder.build().unwrap();

    assert_eq!(*router.dispatch("GET", "/test").unwrap().handler(), "Hello.");
    assert_eq!(*router.dispatch("POST", "/test").unwrap().handler(), "World.");

    assert!(router.dispatch("PUT", "/test").is_none());

    assert!(router.dispatch("GET", "/test-test").is_none());
    assert!(router.dispatch("POST", "/test-test").is_none());
}
