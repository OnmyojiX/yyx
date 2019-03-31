#[macro_use]
extern crate log;

use serde_derive::{Deserialize, Serialize};
use std::fs;
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct YyxConfig {
  pub host: String,
  pub port: u16,
}

impl Default for YyxConfig {
  fn default() -> Self {
    YyxConfig {
      host: "127.0.0.1".to_string(),
      port: 1128,
    }
  }
}

pub fn read_or_create_default() -> YyxConfig {
  match fs::read_to_string("yyx.config.toml") {
    Ok(content) => match toml::from_str(&content) {
      Ok(config) => return config,
      Err(err) => error!("Invalid config file: {}", err),
    },
    Err(err) => {
      error!("Read config file error: {}", err);
    }
  };
  let default_config = Default::default();
  if let Err(err) = fs::write(
    "yyx.config.toml",
    toml::to_string_pretty(&default_config).unwrap(),
  ) {
    error!("Create default config file error: {}", err);
  }
  default_config
}
