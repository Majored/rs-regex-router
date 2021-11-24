// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
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

pub struct Route<'a, V> {
    regex: &'a str,
    params: Vec<&'a str>,
    handlers: HashMap<&'a str, V>,
}

impl<'a, V> Route<'a, V> {
    /// Construct a Route from its base constituents (raw regex and parameter names).
    pub fn new(regex: &'a str, params: Vec<&'a str>) -> Self {
        Route {
            regex,
            params,
            handlers: HashMap::new(),
        }
    }

    /// Inserts (and potentially overrides) a handler for a specific request method.
    pub fn set(&mut self, method: &'a str, handler: V) {
        self.handlers.insert(method, handler);
    }

    /// Returns an immutable reference to the route's raw regex.
    pub fn regex(&self) -> &str {
        self.regex
    }

    /// Returns an immutable string slice of the route's parameter names.
    pub fn params(&self) -> &[&str] {
        &self.params
    }

    /// Returns an immutable reference to the route's map of methods to handlers.
    pub fn handlers(&self) -> &HashMap<&'a str, V> {
        &self.handlers
    }
}

pub struct RouteMatch<'a, V> {
    handler: &'a V,
    params: HashMap<&'a str, &'a str>,
}

impl<'a, V> RouteMatch<'a, V> {
    /// Returns an immutable reference to the matched handler.
    pub fn handler(&self) -> &'a V {
        self.handler
    }

    /// Returns an immutable reference to the extracted path parameters.
    pub fn params<'b>(&'b self) -> &'b HashMap<&'a str, &'a str> {
        &self.params
    }
}

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
            let route = builder.routes.remove(0);
            let param_len = route.params().len();

            routes.push(Some(route));
            for _ in 0..param_len {
                routes.push(None);
            }
        }

        Ok(Self { regex, routes })
    }

    /// Dispatch a request against the routes stored within this router and return a match if found.
    pub fn dispatch<'b>(&'b self, method: &'b str, path: &'b str) -> Option<RouteMatch<'b, V>> {
        let captures = self.regex.captures(&path)?;

        let mut first_match: Option<usize> = None;
        for group in 1..captures.len() {
            if captures.get(group).is_some() {
                first_match = Some(group);
                break;
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

    /// Returns an iterator over the routes defined within this router.
    pub fn routes(&self) -> impl Iterator<Item = &Route<'a, V>> {
        self.routes.iter().filter(|route| route.is_some()).map(|route| route.as_ref().unwrap())
    }
}

pub struct RouterBuilder<'a, V> {
    routes: Vec<Route<'a, V>>,
}

impl<'a, V> RouterBuilder<'a, V> {
    /// Construct an empty builder for Router.
    pub fn new() -> Self {
        RouterBuilder { routes: Vec::new() }
    }

    /// Add a constructed Route to this builder.
    /// 
    /// # Note
    /// Where it is possible for a path to match against multiple routes, the first route to match will always be
    /// returned. Thus, when designing routes specifically around this behaviour, they should be defined in priority
    /// order; any rotues preceding another will have a higher priority.
    pub fn define(&mut self, route: Route<'a, V>) {
        self.routes.push(route);
    }

    /// Consume this builder and construct a Router.
    /// 
    /// # Note
    /// Building may fail if any individual route regex fails to compile, or if this builder is empty.
    pub fn build(self) -> Result<Router<'a, V>, RouterError> {
        Router::new(self)
    }
}

impl<'a, V> Default for RouterBuilder<'a, V> {
    fn default() -> Self {
        Self::new()
    }
}

/// Construct a raw combined regex from individual routes.
fn construct_combined_regex<V>(routes: &[Route<V>]) -> String {
    let mut combined_regex = String::new();

    for route in routes {
        combined_regex.push_str("(^");
        combined_regex.push_str(route.regex);
        combined_regex.push_str("$)|");
    }

    combined_regex.pop();
    combined_regex
}

/// Conveniently construct a Route and define it within a provided RouterBuilder.
/// 
/// # Example
/// ```
/// route!(builder; r"/test";; "GET" => ());
/// route!(builder; r"/test";; "GET" => (), "POST" => ());
/// 
/// route!(builder; r"/test/(.+)"; "example"; "GET" => ());
/// route!(builder; r"/test/(.+)/(.+)"; "example", "var"; "GET" => ());
/// ```
#[macro_export]
macro_rules! route {
    ($builder:ident; $route:expr; $($param:expr),*; $($method:expr => $handler:expr),+) => {{
        let mut route = $crate::Route::new($route, vec![$($param,)*]);
        $(route.set($method, $handler);)+
        $builder.define(route);
    }};
}
