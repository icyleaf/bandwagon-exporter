use crate::bandwagon::Kiwivm;
use crate::command::Command;
use crate::configuration::Configuration;
use crate::metrics;

use std::net::SocketAddr;
use hyper::{
  service::{make_service_fn, service_fn},
  http::header::CONTENT_TYPE,
  Body, Request, Response, StatusCode
};

async fn reqest_metrics(command: &Command) -> String {
  let config = Configuration::from(&command).unwrap();
  let client = Kiwivm::new(config.endpoint);

  for node in &config.nodes {
    println!("{:?}", node);

    let server_info = client.get_service_info(&node.veid, &node.api_key)
      .await
      .expect("Failes to fetch node info.");

    println!("{:?}", server_info);

    metrics::set_node_info(&server_info);
    metrics::set_data_counter(&server_info);
    metrics::set_data_next_reset(&server_info);
    metrics::set_plan_monthly_data(&server_info);

    let rate_limit_status = client.get_rate_limit_status(&node.veid, &node.api_key)
        .await
    .expect("Failes to fetch api rate limit status.");

    metrics::set_api_rate_limit_status(&node, &rate_limit_status);
  }

  String::from(metrics::render_prometheus_text_data())
}

async fn serve_path(
  req: Request<Body>,
  command: Command) -> Response<Body> {
    // Response::new("Hello, World".into())

  let is_get_method = req.method() == "GET";
  let status = if is_get_method {
    StatusCode::NOT_FOUND
  } else {
    StatusCode::METHOD_NOT_ALLOWED
  };

  if !is_get_method || req.uri().path() != command.metrics_path {
    Response::builder()
      .status(status)
      .body(hyper::Body::from(status.canonical_reason().unwrap()))
      .unwrap()
  } else {
    let body = reqest_metrics(&command).await;

    Response::builder()
      .status(StatusCode::OK)
      .header(CONTENT_TYPE, "text/plain")
      .body(Body::from(body))
      .unwrap()
  }
}

pub async fn run_web_server(addr: SocketAddr, command: Command) {
  let metrics_path = command.metrics_path.clone();
  println!("Starting exporter on http://{}{}", addr, metrics_path);
  let command = command.clone();

  let make_service = make_service_fn(move |_| {
    let command = command.clone();

    async move {
      let func = move |req| {
        let command = command.clone();
        async move { Ok::<_, hyper::Error>(serve_path(req, command).await) }
      };

      Ok::<_, hyper::Error>(service_fn(func))
    }
});

  // let make_service = make_service_fn(|_| async {
  //   async move {
  //     let command = command.clone();
  //     Ok::<_, hyper::Error>(service_fn(|req| async {
  //       serve_path(req, command.clone()).await
  //     }))
  //   }
  // });

  let server = hyper::Server::bind(&addr)
    .serve(make_service);

  // Run this server for... forever!
  if let Err(e) = server.await {
      eprintln!("server error: {}", e);
  }
}
