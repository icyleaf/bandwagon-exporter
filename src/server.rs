#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use std::net::SocketAddr;
use std::convert::Infallible;
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use hyper::http::header::CONTENT_TYPE;

async fn hello_world(req: Request<Body>, ) -> Result<Response<Body>, hyper::http::Error> {
  trace!(
    "serve_function:: req.uri() == {}, req.method() == {}",
    req.uri().path(),
    req.method()
  );

  if req.uri().path() != "/metrics" {
    Response::builder()
      .status(StatusCode::NOT_FOUND)
      .body(hyper::Body::empty())
  } else if req.method() != "GET" {
    Response::builder()
      .status(StatusCode::METHOD_NOT_ALLOWED)
      .body(hyper::Body::empty())
  } else {
    Response::builder()
      .status(StatusCode::OK)
      .header(CONTENT_TYPE, "text/plain")
      .body(Body::from("dsfsadfasdfsadfsadfsd"))
    // return Ok(match f(req, options).await {
    //   Ok(response) => Response::builder()
    //     .status(StatusCode::OK)
    //     .header(CONTENT_TYPE, "text/plain; version=0.0.4")
    //     .body(Body::from(response))
    //     .unwrap(),

    //   Err(err) => {
    //     warn!("internal server error == {:?}", err);

    //     Response::builder()
    //       .status(StatusCode::INTERNAL_SERVER_ERROR)
    //       .body(Body::from(err.to_string()))
    //       .unwrap()
    //   }
    // });
  }
}

pub async fn run_server(addr: SocketAddr) {
  // We'll bind to 127.0.0.1:3000
  // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

  println!("Starting exporter on http://{}/metrics", addr);

  // A `Service` is needed for every connection, so this
  // creates one from our `hello_world` function.
  let make_svc = make_service_fn(|_conn| async {
      // service_fn converts our function into a `Service`
      Ok::<_, Infallible>(service_fn(hello_world))
  });

  let server = Server::bind(&addr).serve(make_svc);

  // Run this server for... forever!
  if let Err(e) = server.await {
      eprintln!("server error: {}", e);
  }
}
