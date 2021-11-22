// Copyright (c) 2021 MetricsPage [Harry Plumb]
// MIT License (https://github.com/MetricsPage/hyper-regex-router/blob/main/LICENSE)

//! https://www.npopov.com/2014/02/18/Fast-request-routing-using-regular-expressions.html
//!
//! Whilst using RegexSet would've been easier to reason about, it doesn't support the retrieval of specific group
//! matches within the sub-regexes. Thus, we wouldn't be able to support path parameters and the extraction of them.

#[cfg(test)]
pub mod tests;
pub mod error;

use error::RouterError;
use std::collections::HashMap;
use regex::Regex;

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
        Route { regex, params, handlers }
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
    ///
    fn new(builder: RouterBuilder<'a, V>) -> Result<Self, RouterError> {
        unimplemented!();
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

        if first_match.is_none() {
            return None;
        }

        let route = self.routes.get(first_match.unwrap() - 1).unwrap().as_ref().unwrap();
        let handler = route.handlers.get(method)?;
        let mut params: HashMap<&str, &str> = HashMap::with_capacity(route.params.len());

        for i in 0..route.params().len() {
            let key = route.params().get(i).unwrap();
            let value = captures.get(first_match.unwrap() + i + 1).unwrap();

            params.insert(key, value.as_str());
        }

        Some(RouteMatch {
            handler,
            params,
        })
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
