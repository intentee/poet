use std::fs;

use anyhow::{Error, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configuration {
    #[serde(default)]
    pub sitemap: bool,
    #[serde(default)]
    pub robots: bool,
}

pub fn parse_configuration_file(config_path: &String) -> Result<Configuration, Error> {
    let configuration_file: String = fs::read_to_string(config_path)?;
    let config: Configuration = toml::from_str(&configuration_file)?;

    Ok(config)
}
