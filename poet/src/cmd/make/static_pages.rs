use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use log::info;

use crate::asset_path_renderer::AssetPathRenderer;
use crate::build_authors::build_authors;
use crate::build_project::build_project;
use crate::build_project::build_project_params::BuildProjectParams;
use crate::build_project::build_project_result_stub::BuildProjectResultStub;
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

    #[arg(long, default_value = "false")]
    sitemap: bool,

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
        let authors = build_authors(source_filesystem.clone()).await?;

        let BuildProjectResultStub {
            esbuild_metafile,
            memory_filesystem,
            ..
        } = build_project(BuildProjectParams {
            asset_path_renderer: AssetPathRenderer {
                base_path: self.public_path.clone(),
            },
            authors,
            esbuild_metafile: read_esbuild_metafile_or_default(source_filesystem.clone()).await?,
            generated_page_base_path: self.public_path.clone(),
            generate_sitemap: self.sitemap,
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

        copy_esbuild_metafile_assets_to(
            esbuild_metafile,
            &self.source_directory,
            &self.output_directory,
        )
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;
    use tokio::fs;

    use super::*;

    #[tokio::test]
    async fn copies_esbuild_assets_from_source_directory() -> Result<()> {
        let source_directory = tempdir()?;
        let output_directory = tempdir()?;

        fs::create_dir_all(source_directory.path().join("assets")).await?;
        fs::write(
            source_directory.path().join("assets/app_ABCDEF12.css"),
            "body{}",
        )
        .await?;
        fs::write(
            source_directory.path().join("esbuild-meta.json"),
            r#"{"outputs":{"assets/app_ABCDEF12.css":{"imports":[],"entryPoint":"resources/app.css","inputs":{}}}}"#,
        )
        .await?;

        StaticPages {
            output_directory: output_directory.path().to_path_buf(),
            public_path: "/".to_string(),
            sitemap: false,
            source_directory: source_directory.path().to_path_buf(),
        }
        .handle()
        .await?;

        assert_eq!(
            fs::read_to_string(output_directory.path().join("assets/app_ABCDEF12.css")).await?,
            "body{}"
        );

        Ok(())
    }
}
