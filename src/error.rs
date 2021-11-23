// Copyright (c) 2021 MetricsPage [Harry Plumb]
// MIT License (https://github.com/MetricsPage/hyper-regex-router/blob/main/LICENSE)

#[derive(Debug)]
pub enum RouterError {
    RegexError(regex::Error),
    EmptyRouterError,
}

impl From<regex::Error> for RouterError {
    fn from(err: regex::Error) -> Self {
        RouterError::RegexError(err)
    }
}
