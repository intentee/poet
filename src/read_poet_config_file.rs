use std::path::Path;

use anyhow::Result;
use tokio::fs::read_to_string;

use crate::poet_config::PoetConfig;

pub async fn read_poet_config_file(poet_config_path: &Path) -> Result<PoetConfig> {
    let contents = read_to_string(poet_config_path).await?;

    Ok(toml::from_str(&contents)?)
}
