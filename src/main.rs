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
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Produce various output formats based on your content files
    Make {
        #[command(subcommand)]
        command: Make,
    },
    /// Serves the application, starts MCP server from AppDir (run `poet make app-dir` first)
    Serve(Serve),
    /// Starts Poet in watch mode, and built-in MCP server
    Watch(Watch),
}

#[derive(Subcommand)]
enum Make {
    /// Generates AppDir (packageable with AppImageKit)
    AppDir(AppDir),
    /// Generates static pages
    StaticPages(StaticPages),
}

fn get_handler() -> Option<Box<dyn Handler>> {
    match Cli::parse().command {
        Some(Commands::Make { command }) => match command {
            Make::AppDir(handler) => Some(Box::new(handler)),
            Make::StaticPages(handler) => Some(Box::new(handler)),
        },
        Some(Commands::Serve(handler)) => Some(Box::new(handler)),
        Some(Commands::Watch(handler)) => Some(Box::new(handler)),
        None => None,
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .filter_module("tantivy", log::LevelFilter::Warn)
        .init();

    if let Some(handler) = get_handler() {
        handler.handle().await
    } else {
        Ok(())
    }
}
