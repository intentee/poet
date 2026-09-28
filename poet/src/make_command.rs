use clap::Subcommand;

use crate::cmd::handler::Handler;
use crate::cmd::make::app_dir::AppDir;
use crate::cmd::make::static_pages::StaticPages;

#[derive(Subcommand)]
pub enum MakeCommand {
    #[command(about = "Generates AppDir (packageable with AppImageKit)")]
    AppDir(AppDir),
    #[command(about = "Generates static pages")]
    StaticPages(StaticPages),
}

impl MakeCommand {
    #[must_use]
    pub fn into_handler(self) -> Box<dyn Handler> {
        match self {
            Self::AppDir(handler) => Box::new(handler),
            Self::StaticPages(handler) => Box::new(handler),
        }
    }
}
