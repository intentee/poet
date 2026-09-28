use anyhow::Result;
use clap::Parser as _;
use env_logger::Builder;
use env_logger::Env;
use log::LevelFilter;
use poet::cli::Cli;

#[actix_web::main]
async fn main() -> Result<()> {
    Builder::from_env(Env::default().default_filter_or("info"))
        .filter_module("tantivy", LevelFilter::Warn)
        .init();

    Ok(Cli::parse().command.into_handler().handle().await?)
}
