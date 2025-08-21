use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;

use super::Handler;
use super::value_parser::validate_is_directory;

#[derive(Parser)]
pub struct Generate {
    #[arg(value_parser = validate_is_directory)]
    source_directory: PathBuf,
}

#[async_trait]
impl Handler for Generate {
    async fn handle(&self) -> Result<()> {
        Ok(())
    }
}
