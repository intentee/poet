use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use log::info;

use crate::asset_path_renderer::AssetPathRenderer;
use crate::build_project::BuildProjectParams;
use crate::build_project::BuildProjectResultStub;
use crate::build_project::build_project;
use crate::cmd::builds_project::BuildsProject;
use crate::cmd::handler::Handler;
use crate::cmd::value_parser::validate_is_directory;
use crate::cmd::value_parser::validate_is_directory_or_create;
use crate::compile_shortcodes::compile_shortcodes;
use crate::copy_esbuild_metafile_assets_to::copy_esbuild_metafile_assets_to;
use crate::filesystem::Filesystem;
use crate::filesystem::storage::Storage;
use crate::read_esbuild_metafile_or_default::read_esbuild_metafile_or_default;

#[derive(Parser)]
pub struct StaticPages {
    #[arg(long, value_parser = validate_is_directory_or_create)]
    output_directory: PathBuf,

    #[arg(long)]
    public_path: String,

    #[arg(value_parser = validate_is_directory)]
    source_directory: PathBuf,
}

impl BuildsProject for StaticPages {
    fn source_directory(&self) -> PathBuf {
        self.source_directory.clone()
    }
}

#[async_trait(?Send)]
impl Handler for StaticPages {
    async fn handle(&self) -> Result<()> {
        let source_filesystem = self.source_filesystem();
        let rhai_template_renderer = compile_shortcodes(source_filesystem.clone()).await?;
        let BuildProjectResultStub {
            esbuild_metafile,
            memory_filesystem,
            ..
        } = build_project(BuildProjectParams {
            asset_path_renderer: AssetPathRenderer {
                base_path: self.public_path.clone(),
            },
            esbuild_metafile: read_esbuild_metafile_or_default(source_filesystem.clone()).await?,
            generated_page_base_path: self.public_path.clone(),
            is_watching: false,
            rhai_template_renderer,
            source_filesystem,
        })
        .await?;

        let storage = Storage {
            base_directory: self.output_directory.clone(),
        };

        info!("Saving generated files in output directory...");

        storage.copy_project_files_from(memory_filesystem).await?;

        info!("Copying assets into output directory...");

        copy_esbuild_metafile_assets_to(esbuild_metafile, &self.output_directory).await?;

        Ok(())
    }
}
