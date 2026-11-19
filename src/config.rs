use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct Config {
   pub ip: String,
   pub port: u16,
}

pub fn init_config(config_path: &str) -> Result<Config> {
    let default_config = Config {
        ip: "0.0.0.0".to_string(),
        port: 1338
    };
    match fs::read_to_string(config_path) {
        Ok(config_file) => {
            let config = toml::from_str(&config_file)?;
            return Ok(config);
        },
        Err(_) => {
            let config_str = toml::to_string(&default_config)?;
            fs::write(config_path, config_str)?;
            return Ok(default_config)
        },
    }
}