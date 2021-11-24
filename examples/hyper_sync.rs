// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/MetricsPage/hyper-regex-router/blob/main/LICENSE)

use hyper::service::{make_service_fn, service_fn};
use hyper::{Server, Response, Body, Request, Method};
use std::{convert::Infallible, net::SocketAddr, sync::Arc};
use regex_router::{RouterBuilder, Router, route};

type Handler = fn(Request<Body>) -> Result<Response<Body>, Infallible>;

#[tokio::main]
async fn main() {
    let http_addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();

    // Build the request router and wrap it within an Arc.
    let mut router = RouterBuilder::<Handler>::new();
    route!(router; r"/hello";; Method::GET.as_str() => (|_| Ok(Response::new(Body::from("Hello.")))) as Handler);
    route!(router; r"/bye";; Method::GET.as_str() => bye as Handler);
    let router = Arc::new(router.build().unwrap());

    let http_service = make_service_fn(|_| {
        let l_router = router.clone();

        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handler(req, l_router.clone())
            }))
        }
    });

    if let Err(_) = Server::bind(&http_addr).serve(http_service).await {
        std::process::exit(-1);
    }
}

async fn handler(req: Request<Body>, router: Arc<Router<'_, Handler>>) -> Result<Response<Body>, Infallible> {
    match router.dispatch(req.method().as_str(), req.uri().path()) {
        Some(route) => route.handler()(req),
        None => Ok(Response::new(Body::from("404 Error."))),
    }
}

fn bye(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Bye.")))
}