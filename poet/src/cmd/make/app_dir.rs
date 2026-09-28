use std::path::PathBuf;

use async_trait::async_trait;
use clap::Parser;
use poet_app_dir::app_dir_desktop_entry::AppDirDesktopEntry;
use poet_app_dir::app_dir_name::AppDirName;
use poet_app_dir::build_app_dir::build_app_dir;
use poet_app_dir::build_app_dir_params::BuildAppDirParams;
use poet_app_dir::validate_desktop_entry_string::validate_desktop_entry_string;

use crate::cmd::handler::Handler;
use crate::cmd::value_parser::validate_is_directory::validate_is_directory;
use crate::cmd::value_parser::validate_is_directory_or_create::validate_is_directory_or_create;
use crate::poet_error::PoetError;

#[derive(Parser)]
pub struct AppDir {
    #[arg(long)]
    name: String,

    #[arg(long, value_parser = validate_is_directory_or_create)]
    output_directory: PathBuf,

    #[arg(value_parser = validate_is_directory)]
    source_directory: PathBuf,

    #[arg(long)]
    title: String,

    #[arg(long)]
    version: String,
}

#[async_trait(?Send)]
impl Handler for AppDir {
    async fn handle(&self) -> Result<(), PoetError> {
        build_app_dir(BuildAppDirParams {
            desktop_entry: AppDirDesktopEntry {
                name: AppDirName::parse(&self.name)?,
                poet_version: env!("CARGO_PKG_VERSION").to_owned(),
                site_version: validate_desktop_entry_string(&self.version)?,
                title: validate_desktop_entry_string(&self.title)?,
            },
            output_directory: &self.output_directory,
            source_directory: &self.source_directory,
        })
        .await?;

        Ok(())
    }
}
