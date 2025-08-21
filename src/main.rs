use anyhow::Result;
use clap::Parser;
use clap::Subcommand;
use poet::cmd::Handler as _;
use poet::cmd::generate::Generate;

#[derive(Parser)]
#[command(arg_required_else_help(true), version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generates static pages, prepares MCP resources list
    Generate(Generate),
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    match Cli::parse().command {
        Some(Commands::Generate(handler)) => Ok(handler.handle().await?),
        None => Ok(()),
    }
}
