// Copyright (c) 2021 MetricsPage [Harry Plumb]
// MIT License (https://github.com/MetricsPage/hyper-regex-router/blob/main/LICENSE)

//! https://www.npopov.com/2014/02/18/Fast-request-routing-using-regular-expressions.html
//!
//! Whilst using RegexSet would've been easier to reason about, it doesn't support the retrieval of specific group
//! matches within the sub-regexes. Thus, we wouldn't be able to support path parameters and the extraction of them.

pub mod error;

use error::RouterError;

use regex::Regex;

///
pub struct Route<'a, V> {
    regex: &'a str,
    params: Vec<&'a str>,
    handler: V,
}

impl<'a, V> Route<'a, V> {
    ///
    pub fn new(regex: &'a str, params: Vec<&'a str>, handler: V) -> Self {
        Route { regex, params, handler }
    }

    ///
    pub fn regex(&self) -> &str {
        self.regex
    }

    ///
    pub fn params(&self) -> &[&str] {
        &self.params
    }

    ///
    pub fn handler(&self) -> &V {
        &self.handler
    }
}

///
pub struct Router<'a, V> {
    regex: Regex,
    routes: Vec<Option<Route<'a, V>>>,
}

impl<'a, V> Router<'a, V> {
    ///
    fn new(builder: RouterBuilder<'a, V>) -> Result<Self, RouterError> {
        unimplemented!();
    }
}

///
pub struct RouterBuilder<'a, V> {
    routes: Vec<Route<'a, V>>,
}

impl<'a, V> RouterBuilder<'a, V> {
    ///
    pub fn new() -> Self {
        RouterBuilder { routes: Vec::new() }
    }

    ///
    pub fn define(&mut self, route: Route<'a, V>) {
        self.routes.push(route);
    }

    ///
    pub fn build(self) -> Result<Router<'a, V>, RouterError> {
        Router::new(self)
    }
}

impl<'a, V> Default for RouterBuilder<'a, V> {
    fn default() -> Self {
        Self::new()
    }
}
