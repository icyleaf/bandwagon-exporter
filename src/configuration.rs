use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use crate::Command;

#[derive(Debug, Deserialize, Clone)]
pub struct Node {
  pub veid: String,
  pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
  pub endpoint: String,
  pub metrics_path: String,
  pub nodes: Vec<Node>
}

impl Configuration {
  pub fn from(command: &Command) -> Result<Self, ConfigError> {
    let config = Config::builder()
      .add_source(File::with_name(command.config_path.to_str().unwrap()))
      .add_source(Environment::with_prefix("BANDWAGON"))
      .set_default("endpoint", "https://api.64clouds.com/v1").unwrap()
      .set_default("metrics_path", command.metrics_path.clone()).unwrap()
      .build()?;

    config.try_deserialize()
  }
}
