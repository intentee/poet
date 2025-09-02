use anyhow::Result;
use clap::Parser;
use clap::Subcommand;
use poet::cmd::Handler as _;
use poet::cmd::generate::Generate;
use poet::cmd::watch::Watch;

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
    Watch(Watch),
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    match Cli::parse().command {
        Some(Commands::Generate(handler)) => Ok(handler.handle().await?),
        Some(Commands::Watch(handler)) => Ok(handler.handle().await?),
        None => Ok(()),
    }
}
