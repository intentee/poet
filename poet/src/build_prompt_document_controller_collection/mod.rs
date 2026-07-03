pub mod build_prompt_document_controller_collection_params;

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use anyhow::anyhow;
use dashmap::DashMap;
use log::info;
use rayon::iter::IntoParallelIterator as _;
use rayon::iter::ParallelIterator as _;

use crate::build_prompt_document_controller::build_prompt_document_controller;
use crate::build_prompt_document_controller_collection::build_prompt_document_controller_collection_params::BuildPromptControllerCollectionParams;
use crate::build_prompt_document_controller_params::BuildPromptDocumentControllerParams;
use crate::build_timer::BuildTimer;
use crate::document_error_collection::DocumentErrorCollection;
use crate::filesystem::Filesystem as _;
use crate::mcp::prompt_controller::PromptController;
use crate::mcp::prompt_controller_collection::PromptControllerCollection;

pub async fn build_prompt_document_controller_collection(
    BuildPromptControllerCollectionParams {
        asset_path_renderer,
        content_document_linker,
        esbuild_metafile,
        rhai_template_renderer,
        source_filesystem,
    }: BuildPromptControllerCollectionParams,
) -> Result<PromptControllerCollection> {
    info!("Processing prompt files...");

    let _build_timer = BuildTimer::default();
    let error_collection: DocumentErrorCollection = Default::default();
    let prompt_controller_map: DashMap<String, Arc<dyn PromptController>> = Default::default();

    source_filesystem
        .read_project_files()
        .await?
        .into_par_iter()
        .filter(|file| file.kind.is_prompt())
        .for_each(|file| {
            let name = file
                .get_stem_path_relative_to(&PathBuf::from("prompts"))
                .display()
                .to_string();

            match build_prompt_document_controller(BuildPromptDocumentControllerParams {
                asset_path_renderer: asset_path_renderer.clone(),
                content_document_linker: content_document_linker.clone(),
                esbuild_metafile: esbuild_metafile.clone(),
                file,
                name: name.clone(),
                rhai_template_renderer: rhai_template_renderer.clone(),
            }) {
                Ok(prompt_document_controller) => {
                    prompt_controller_map.insert(name, Arc::new(prompt_document_controller));
                }
                Err(err) => {
                    error_collection.register_error(name, err);
                }
            }
        });

    if !error_collection.is_empty() {
        return Err(anyhow!("{error_collection}"));
    }

    Ok(prompt_controller_map.into())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use tempfile::tempdir;

    use super::*;
    use crate::asset_path_renderer::AssetPathRenderer;
    use crate::compile_shortcodes::compile_shortcodes;
    use crate::filesystem::storage::Storage;
    use crate::mcp::list_resources_cursor::ListResourcesCursor;

    async fn build(prompt_files: &[(&str, &str)]) -> Result<PromptControllerCollection> {
        let directory = tempdir()?;
        let source_filesystem = Arc::new(Storage {
            base_directory: directory.path().to_path_buf(),
        });

        for (relative_path, contents) in prompt_files {
            source_filesystem
                .set_file_contents(Path::new(relative_path), contents)
                .await?;
        }

        let rhai_template_renderer = compile_shortcodes(source_filesystem.clone()).await?;

        build_prompt_document_controller_collection(BuildPromptControllerCollectionParams {
            asset_path_renderer: AssetPathRenderer {
                base_path: "/".to_string(),
            },
            content_document_linker: Default::default(),
            esbuild_metafile: Default::default(),
            rhai_template_renderer,
            source_filesystem,
        })
        .await
    }

    #[tokio::test]
    async fn builds_a_controller_for_each_prompt_file() -> Result<()> {
        let collection = build(&[(
            "prompts/greet.md",
            "+++\narguments = {}\ndescription = \"Greeting\"\ntitle = \"Greet\"\n+++\n\n**user**: hello\n",
        )])
        .await?;

        let prompts = collection.list_mcp_prompts(ListResourcesCursor {
            offset: 0,
            per_page: 10,
        });

        assert_eq!(prompts.len(), 1);
        assert_eq!(prompts[0].name, "greet");

        Ok(())
    }

    #[tokio::test]
    async fn aggregates_errors_from_invalid_prompt_front_matter() {
        let outcome = build(&[(
            "prompts/broken.md",
            "+++\ntitle = \"Missing required fields\"\n+++\n\n**user**: hi\n",
        )])
        .await;

        assert!(outcome.is_err());
    }
}
