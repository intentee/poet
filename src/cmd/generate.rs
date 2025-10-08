use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use log::info;
use tokio::fs;

use super::Handler;
use super::value_parser::validate_is_directory;
use super::value_parser::validate_is_directory_or_create;
use crate::asset_path_renderer::AssetPathRenderer;
use crate::build_project::build_project;
use crate::build_project::build_project_result::BuildProjectResult;
use crate::cmd::builds_project::BuildsProject;
use crate::compile_shortcodes::compile_shortcodes;
use crate::filesystem::Filesystem;
use crate::filesystem::storage::Storage;
use crate::filesystem::storage::create_parent_directories::create_parent_directories;

#[derive(Parser)]
pub struct Generate {
    #[arg(long, value_parser = validate_is_directory_or_create)]
    output_directory: PathBuf,

    #[arg(long)]
    public_path: String,

    #[arg(value_parser = validate_is_directory)]
    source_directory: PathBuf,
}

impl BuildsProject for Generate {
    fn source_directory(&self) -> PathBuf {
        self.source_directory.clone()
    }
}

#[async_trait]
impl Handler for Generate {
    async fn handle(&self) -> Result<()> {
        let source_filesystem = self.source_filesystem();
        let rhai_template_renderer = compile_shortcodes(source_filesystem.clone()).await?;

        let BuildProjectResult {
            esbuild_metafile,
            markdown_document_sources: _,
            memory_filesystem,
        } = build_project(
            AssetPathRenderer {
                base_path: self.public_path.clone(),
            },
            self.public_path.clone(),
            false,
            rhai_template_renderer,
            source_filesystem,
        )
        .await?;

        let storage = Storage {
            base_directory: self.output_directory.clone(),
        };

        info!("Saving generated files in output directory...");

        storage.copy_from(memory_filesystem).await?;

        info!("Copying assets into output directory...");

        for asset_path in esbuild_metafile.get_output_paths().iter() {
            let target_path = self.output_directory.join(asset_path);

            create_parent_directories(&target_path).await?;

            fs::copy(asset_path, target_path).await?;
        }

        Ok(())
    }
}
