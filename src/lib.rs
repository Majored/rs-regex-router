// Copyright (c) 2021 MetricsPage [Harry Plumb]
// MIT License (https://github.com/MetricsPage/hyper-regex-router/blob/main/LICENSE)

//! https://www.npopov.com/2014/02/18/Fast-request-routing-using-regular-expressions.html
//!
//! Whilst using RegexSet would've been easier to reason about, it doesn't support the retrieval of specific group
//! matches within the sub-regexes. Thus, we wouldn't be able to support path parameters and the extraction of them.

pub mod error;
#[cfg(test)]
pub mod tests;

use error::RouterError;
use regex::Regex;
use std::collections::HashMap;

///
pub struct Route<'a, V> {
    regex: &'a str,
    params: Vec<&'a str>,
    handlers: HashMap<&'a str, V>,
}

pub struct RouteMatch<'a, V> {
    handler: &'a V,
    params: HashMap<&'a str, &'a str>,
}

impl<'a, V> Route<'a, V> {
    ///
    pub fn new(regex: &'a str, params: Vec<&'a str>, handlers: HashMap<&'a str, V>) -> Self {
        Route {
            regex,
            params,
            handlers,
        }
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
    pub fn handlers(&self) -> &HashMap<&'a str, V> {
        &self.handlers
    }
}

///
pub struct Router<'a, V> {
    regex: Regex,
    routes: Vec<Option<Route<'a, V>>>,
}

impl<'a, V> Router<'a, V> {
    /// Construct a Router from its constituents stored within a builder.
    fn new(mut builder: RouterBuilder<'a, V>) -> Result<Self, RouterError> {
        if builder.routes.is_empty() {
            return Err(RouterError::EmptyRouterError);
        }

        let mut routes = Vec::new();
        let combined_regex = construct_combined_regex(&builder.routes);
        let regex = Regex::new(&combined_regex)?;

        while !builder.routes.is_empty() {
            let route = builder.routes.pop().unwrap();
            let param_len = route.params().len();

            routes.push(Some(route));
            for _ in 0..param_len {
                routes.push(None);
            }
        }

        Ok(Self { regex, routes })
    }

    ///
    pub fn dispatch<'b>(&'b self, method: &'b str, path: &'b str) -> Option<RouteMatch<'b, V>> {
        let captures = self.regex.captures(&path)?;

        let mut first_match: Option<usize> = None;
        for group in 1..captures.len() {
            if captures.get(group).is_some() {
                first_match = Some(group);
            }
        }

        let route = self.routes.get(first_match? - 1).unwrap().as_ref().unwrap();
        let handler = route.handlers.get(method)?;
        let mut params: HashMap<&str, &str> = HashMap::with_capacity(route.params.len());

        for i in 0..route.params().len() {
            let key = route.params().get(i).unwrap();
            let value = captures.get(first_match? + i + 1).unwrap().as_str();

            params.insert(key, value);
        }

        Some(RouteMatch { handler, params })
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

fn construct_combined_regex<V>(routes: &[Route<V>]) -> String {
    let mut combined_regex = String::new();

    for route in routes {
        combined_regex.push_str("(");
        combined_regex.push_str(route.regex);
        combined_regex.push_str(")|");
    }

    if combined_regex.chars().last().unwrap() == '|' {
        combined_regex.pop();
    }

    combined_regex
}
