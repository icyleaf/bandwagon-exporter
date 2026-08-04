use crate::bandwagon::Kiwivm;
use crate::command::Command;
use crate::configuration::{Configuration, Node};
use crate::metrics;

use std::net::SocketAddr;
use bytes::Bytes;
use http_body_util::Full;
use hyper::{
  body::Incoming,
  http::header::CONTENT_TYPE,
  server::conn::http1,
  service::service_fn,
  Request, Response, StatusCode
};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

async fn request_server_info(client: Kiwivm, node: Node) {
  let server_info = client.get_service_info(&node.veid, &node.api_key)
    .await
    .expect("Failes to fetch node info.");

  metrics::inc_api_request_total(&node);
  metrics::set_node_info(&server_info);
  metrics::set_data_counter(&server_info);
  metrics::set_data_next_reset(&server_info);
  metrics::set_plan_monthly_data(&server_info);
}

async fn request_api_rate_limit(client: Kiwivm, node: Node) {
  let rate_limit_status = client.get_rate_limit_status(&node.veid, &node.api_key)
  .await
  .expect("Failes to fetch api rate limit status.");

  metrics::inc_api_request_total(&node);
  metrics::set_api_rate_limit_status(&node, &rate_limit_status);
}

async fn reqest_metrics(command: &Command) -> String {
  let config = Configuration::from(command).unwrap();
  let client = Kiwivm::new(config.endpoint);

  for node in config.nodes {
    tokio::spawn(request_server_info(client.clone(), node.clone()));
    tokio::spawn(request_api_rate_limit(client.clone(), node.clone()));
  }

  metrics::render_prometheus_text_data()
}

async fn serve_path(
  req: Request<Incoming>,
  command: Command) -> Response<Full<Bytes>> {

  let is_get_method = req.method() == "GET";
  let status = if is_get_method {
    StatusCode::NOT_FOUND
  } else {
    StatusCode::METHOD_NOT_ALLOWED
  };

  if !is_get_method || req.uri().path() != command.metrics_path {
    let body = format!("<h3>Prometheus Bandwagon Exporter version {} by {}.</h3><p>Path: <a href='/metrics'>/metrics</a></p>", VERSION, AUTHOR);

    Response::builder()
      .status(status)
      .header(CONTENT_TYPE, "text/html")
      .body(Full::new(Bytes::from(body)))
      .unwrap()
  } else {
    let body = reqest_metrics(&command).await;

    Response::builder()
      .status(StatusCode::OK)
      .header(CONTENT_TYPE, "text/plain")
      .body(Full::new(Bytes::from(body)))
      .unwrap()
  }
}

pub async fn run_web_server(addr: SocketAddr, command: Command) {
  let command = command.clone();
  println!("Starting exporter on http://{}{}", addr, command.metrics_path);

  let listener = TcpListener::bind(addr)
    .await
    .expect("Failed to bind metrics server address");

  loop {
    let (stream, _) = listener.accept()
      .await
      .expect("Failed to accept connection");
    let io = TokioIo::new(stream);
    let command = command.clone();

    tokio::spawn(async move {
      let func = move |req| {
        let command = command.clone();
        async move { Ok::<_, hyper::Error>(serve_path(req, command).await) }
      };

      if let Err(e) = http1::Builder::new()
        .serve_connection(io, service_fn(func))
        .await
      {
        eprintln!("Failed to serve connection: {}", e);
      }
    });
  }
}
