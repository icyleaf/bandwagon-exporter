#[allow(unused_variables)]
#[allow(unused_imports)]

use std::fs::read_dir;
use std::net::SocketAddr;

use clap::Parser;

mod command;
use command::Command;

mod configuration;
use configuration::Configuration;

mod client;
use client::Kiwivm;

mod metrics;
mod server;

use std::env;

type MainResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> MainResult<()> {

  env::set_var(
    "RUST_LOG",
    format!("folder_size=trace,{}=trace", "sdfsdf"),
  );

  let command = Command::parse();
  // println!("{:?}", command);
  // println!("config_path: {:?}", command.config_path);
  // println!("metrics_server: {:?}", command.metrics_server);

  let configuration = Configuration::new(command.config_path)
    .unwrap();
  // println!("{:?}", configuration);

  let client = Kiwivm::new(configuration.endpoint);
  for server in configuration.servers {
    // println!("{:?}", server);
    let server_info = client.get_service_info(&server.veid, &server.api_key)
      .await
      .expect("Failes to fetch node info.");

    // println!("{:?}", server_info);

    metrics::set_node_info(&server_info);
    metrics::set_data_counter(&server_info);
    metrics::set_data_next_reset(&server_info);
    metrics::set_plan_monthly_data(&server_info);

    let rate_limit_status = client.get_rate_limit_status(&server.veid, &server.api_key)
        .await
    .expect("Failes to fetch api rate limit status.");

    metrics::set_api_rate_limit_status(&server, &rate_limit_status);
  }

  let addr: SocketAddr = command.metrics_server
    .parse()
    .expect("Unable to parse metrics server");

  println!("{}", metrics::render_prometheus_text_data());

  server::run_server(addr).await;

  Ok(())
}
