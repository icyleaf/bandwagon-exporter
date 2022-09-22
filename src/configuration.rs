use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Server {
  pub veid: String,
  pub api_key: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Configuration {
  pub endpoint: String,
  pub servers: Vec<Server>
}

impl Configuration {
  pub fn new(config_path: PathBuf) -> Result<Self, ConfigError> {
    let config = Config::builder()
      .add_source(File::with_name(config_path.to_str().unwrap()))
      .add_source(Environment::with_prefix("BANDWAGON"))
      .set_default("endpoint", "https://api.64clouds.com/v1").unwrap()
      .build()?;

    config.try_deserialize()
  }
}
