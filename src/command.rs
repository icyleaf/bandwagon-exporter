use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Command {
  #[clap(required = true, help = "Config path, avaiables in .y(a)ml, .json, .toml formatted file.", value_parser)]
  pub config_path: PathBuf,

  #[clap(short, long, value_name = "HOST:PORT", default_value_t = String::from("0.0.0.0:9103"), help = "Metrics server", value_parser)]
  pub metrics_server: String,

  #[clap(long, value_name = "path", default_value_t = String::from("/metrics"), help = "The path of metrics server", value_parser)]
  pub metrics_path: String,
}

