use clap::Subcommand;

use crate::cmd::handler::Handler;
use crate::cmd::serve::Serve;
use crate::cmd::watch::Watch;
use crate::make_command::MakeCommand;

#[derive(Subcommand)]
pub enum CliCommand {
    #[command(about = "Produce various output formats based on your content files")]
    Make {
        #[command(subcommand)]
        command: MakeCommand,
    },
    #[command(
        about = "Serves the application, starts MCP server from AppDir (run `poet make app-dir` first)"
    )]
    Serve(Serve),
    #[command(about = "Starts Poet in watch mode, and built-in MCP server")]
    Watch(Watch),
}

impl CliCommand {
    #[must_use]
    pub fn into_handler(self) -> Box<dyn Handler> {
        match self {
            Self::Make { command } => command.into_handler(),
            Self::Serve(handler) => Box::new(handler),
            Self::Watch(handler) => Box::new(handler),
        }
    }
}
