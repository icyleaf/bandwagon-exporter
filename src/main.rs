mod configuration;
mod bandwagon;
mod metrics;
mod server;
mod command;
use command::Command;

use std::net::SocketAddr;
use clap::Parser;

type MainResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> MainResult<()> {
  let command = Command::parse();
  let addr: SocketAddr = command.metrics_server
    .parse()
    .expect("Unable to parse metrics server");

  server::run_web_server(addr, command.clone()).await;

  Ok(())
}
