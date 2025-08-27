use std::path::PathBuf;
use std::str::FromStr as _;
use std::sync::Arc;

use anyhow::Result;
use anyhow::anyhow;
use esbuild_metafile::EsbuildMetaFile;
use log::info;
use log::warn;
use syntect::parsing::SyntaxSet;

use crate::eval_mdast::eval_mdast;
use crate::filesystem::Filesystem;
use crate::filesystem::memory::Memory;
use crate::filesystem::read_file_contents_result::ReadFileContentsResult;
use crate::filesystem::storage::Storage;
use crate::rhai_component_context::RhaiComponentContext;
use crate::rhai_template_factory::RhaiTemplateFactory;
use crate::rhai_template_renderer::RhaiTemplateRenderer;
use crate::string_to_mdast::string_to_mdast;

pub async fn build_project(source_filesystem: &Storage) -> Result<Memory> {
    let esbuild_metafile: Arc<EsbuildMetaFile> = match source_filesystem
        .read_file_contents(&PathBuf::from("esbuild-meta.json"))
        .await?
    {
        ReadFileContentsResult::Directory => {
            return Err(anyhow!(
                "esbuild metafile should be a file, not a directory"
            ));
        }
        ReadFileContentsResult::Found(contents) => EsbuildMetaFile::from_str(&contents)?,
        ReadFileContentsResult::NotFound => {
            warn!("esbuild metafile not found, proceeding without it");

            EsbuildMetaFile::from_str(
                r#"{
                "outputs": {}
            }"#,
            )?
        }
    }
    .into();
    let files = source_filesystem.read_project_files().await?;
    let rhai_template_factory = RhaiTemplateFactory::new(
        source_filesystem.base_directory.clone(),
        PathBuf::from("shortcodes"),
    );
    let syntax_set = SyntaxSet::load_defaults_newlines();

    // First pass, process Rhai files to be used as shortcodes or layouts
    for file in &files {
        if file.is_rhai() {
            info!("Processing shortcode file: {:?}", file.relative_path);

            rhai_template_factory.register_component_file(file.clone());
        }
    }

    info!("Compiling the templates...");

    let rhai_template_renderer: RhaiTemplateRenderer = rhai_template_factory.try_into()?;

    for file in &files {
        if file.is_markdown() {
            info!("Processing content file: {:?}", file.relative_path);

            let mdast = string_to_mdast(&file.contents)?;
            let rhai_component_context = RhaiComponentContext {};

            println!(
                "{}",
                eval_mdast(
                    &mdast,
                    &rhai_component_context,
                    &rhai_template_renderer,
                    &syntax_set
                )?,
            );
        }
    }

    Err(anyhow!("Not implemented yet"))
}
