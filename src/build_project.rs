use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr as _;
use std::sync::Arc;

use anyhow::Result;
use anyhow::anyhow;
use esbuild_metafile::EsbuildMetaFile;
use log::info;
use log::warn;
use rhai::Dynamic;
use syntect::parsing::SyntaxSet;

use crate::asset_manager::AssetManager;
use crate::eval_mdast::eval_mdast;
use crate::filesystem::Filesystem;
use crate::filesystem::memory::Memory;
use crate::filesystem::read_file_contents_result::ReadFileContentsResult;
use crate::filesystem::storage::Storage;
use crate::find_front_matter_in_mdast::find_front_matter_in_mdast;
use crate::markdown_document::MarkdownDocument;
use crate::markdown_document_reference::MarkdownDocumentReference;
use crate::rhai_component_context::RhaiComponentContext;
use crate::rhai_template_factory::RhaiTemplateFactory;
use crate::rhai_template_renderer::RhaiTemplateRenderer;
use crate::string_to_mdast::string_to_mdast;

pub async fn build_project(is_watching: bool, source_filesystem: &Storage) -> Result<Memory> {
    let memory_filesystem = Memory::default();
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

            EsbuildMetaFile::default()
        }
    }
    .into();
    let files = source_filesystem.read_project_files().await?;
    let rhai_template_factory = RhaiTemplateFactory::new(
        source_filesystem.base_directory.clone(),
        PathBuf::from("shortcodes"),
    );
    let syntax_set = SyntaxSet::load_defaults_newlines();

    for file in &files {
        if file.is_rhai() {
            info!("Processing shortcode file: {:?}", file.relative_path);

            rhai_template_factory.register_component_file(file.clone());
        }
    }

    info!("Compiling the templates...");

    let rhai_template_renderer: RhaiTemplateRenderer = rhai_template_factory.try_into()?;

    info!("Processing content files...");

    let mut markdown_document_index: HashMap<String, MarkdownDocumentReference> = HashMap::new();
    let mut markdown_document_list: Vec<MarkdownDocument> = Vec::new();

    for file in &files {
        if file.is_markdown() {
            info!("Processing content file: {:?}", file.relative_path);

            let mdast = string_to_mdast(&file.contents)?;
            let front_matter = find_front_matter_in_mdast(&mdast)?.ok_or_else(|| {
                anyhow!("No front matter found in file: {:?}", file.relative_path)
            })?;

            let basename_path = file.get_stem_path_relative_to(&PathBuf::from("content"));
            let basename = basename_path.display().to_string();

            let markdown_document_reference = MarkdownDocumentReference {
                basename: basename.clone(),
                basename_path,
                front_matter,
            };

            markdown_document_index.insert(basename, markdown_document_reference.clone());
            markdown_document_list.push(MarkdownDocument {
                mdast,
                reference: markdown_document_reference,
            });
        }
    }

    let markdown_document_index_arc = Arc::new(markdown_document_index);

    for MarkdownDocument {
        mdast,
        reference:
            reference @ MarkdownDocumentReference {
                basename,
                basename_path,
                front_matter,
            },
    } in &markdown_document_list
    {
        let rhai_component_context = RhaiComponentContext {
            asset_manager: AssetManager::from_esbuild_metafile(esbuild_metafile.clone()),
            is_watching,
            front_matter: front_matter.clone(),
            markdown_document_index: markdown_document_index_arc.clone(),
        };

        let layout_content = eval_mdast(
            mdast,
            &rhai_component_context,
            &rhai_template_renderer,
            &syntax_set,
        )?;

        let processed_file = rhai_template_renderer.render(
            &front_matter.layout,
            rhai_component_context.clone(),
            Dynamic::from_map(front_matter.props.clone()),
            layout_content.into(),
        )?;

        memory_filesystem
            .set_file_contents(&reference.target_file_relative_path(), &processed_file)
            .await?;
    }

    Ok(memory_filesystem)
}
