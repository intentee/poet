use anyhow::Result;
use clap::Parser;
use clap::Subcommand;
use poet::cmd::handler::Handler;
use poet::cmd::make::app_dir::AppDir;
use poet::cmd::make::static_pages::StaticPages;
use poet::cmd::serve::Serve;
use poet::cmd::watch::Watch;

#[derive(Parser)]
#[command(arg_required_else_help(true), version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Produce various output formats based on your content files")]
    Make {
        #[command(subcommand)]
        command: Make,
    },
    #[command(
        about = "Serves the application, starts MCP server from AppDir (run `poet make app-dir` first)"
    )]
    Serve(Serve),
    #[command(about = "Starts Poet in watch mode, and built-in MCP server")]
    Watch(Watch),
}

#[derive(Subcommand)]
enum Make {
    #[command(about = "Generates AppDir (packageable with AppImageKit)")]
    AppDir(AppDir),
    #[command(about = "Generates static pages")]
    StaticPages(StaticPages),
}

fn command_handler(command: Commands) -> Box<dyn Handler> {
    match command {
        Commands::Make {
            command: Make::AppDir(handler),
        } => Box::new(handler),
        Commands::Make {
            command: Make::StaticPages(handler),
        } => Box::new(handler),
        Commands::Serve(handler) => Box::new(handler),
        Commands::Watch(handler) => Box::new(handler),
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .filter_module("tantivy", log::LevelFilter::Warn)
        .init();

    Ok(command_handler(Cli::parse().command).handle().await?)
}
