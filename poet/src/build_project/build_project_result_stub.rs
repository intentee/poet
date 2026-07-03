use std::collections::BTreeMap;
use std::sync::Arc;

use esbuild_metafile::EsbuildMetaFile;
use rayon::iter::ParallelBridge as _;
use rayon::iter::ParallelIterator as _;

use crate::build_project::build_project_result::BuildProjectResult;
use crate::content_document_basename::ContentDocumentBasename;
use crate::content_document_linker::ContentDocumentLinker;
use crate::content_document_source::ContentDocumentSource;
use crate::filesystem::memory::Memory;

pub struct BuildProjectResultStub {
    pub content_document_linker: ContentDocumentLinker,
    pub content_document_sources: Arc<BTreeMap<ContentDocumentBasename, ContentDocumentSource>>,
    pub esbuild_metafile: Arc<EsbuildMetaFile>,
    pub memory_filesystem: Arc<Memory>,
}

impl BuildProjectResultStub {
    pub fn changed_compared_to(self, other: BuildProjectResult) -> BuildProjectResult {
        let changed_since_last_build: Vec<ContentDocumentSource> = self
            .content_document_sources
            .values()
            .par_bridge()
            .filter(|content_document_source| {
                for other_content_document_source in other.content_document_sources.values() {
                    if other_content_document_source.reference.basename_path
                        == content_document_source.reference.basename_path
                    {
                        return other_content_document_source.file_entry.contents_hash
                            != content_document_source.file_entry.contents_hash;
                    }
                }

                false
            })
            .map(|content_document_source| content_document_source.clone())
            .collect();

        BuildProjectResult {
            changed_since_last_build,
            content_document_linker: self.content_document_linker,
            content_document_sources: self.content_document_sources,
            esbuild_metafile: self.esbuild_metafile,
            memory_filesystem: self.memory_filesystem,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use anyhow::Result;
    use tempfile::tempdir;

    use super::*;
    use crate::asset_path_renderer::AssetPathRenderer;
    use crate::build_authors::build_authors;
    use crate::build_project::build_project;
    use crate::build_project::build_project_params::BuildProjectParams;
    use crate::compile_shortcodes::compile_shortcodes;
    use crate::filesystem::Filesystem as _;
    use crate::filesystem::storage::Storage;

    async fn build_stub(body: &str) -> Result<BuildProjectResultStub> {
        let directory = tempdir()?;
        let source_filesystem = Arc::new(Storage {
            base_directory: directory.path().to_path_buf(),
        });

        source_filesystem
            .set_file_contents(
                Path::new("shortcodes/Layout.rhai"),
                "fn template(context, props, content) { component { <html>{content}</html> } }",
            )
            .await?;
        source_filesystem
            .set_file_contents(
                Path::new("content/guide.md"),
                &format!(
                    "+++\ndescription = \"Guide\"\nlayout = \"Layout\"\ntitle = \"Guide\"\n+++\n\n{body}\n"
                ),
            )
            .await?;

        let rhai_template_renderer = compile_shortcodes(source_filesystem.clone()).await?;
        let authors = build_authors(source_filesystem.clone()).await?;

        build_project(BuildProjectParams {
            asset_path_renderer: AssetPathRenderer {
                base_path: "/".to_string(),
            },
            authors,
            esbuild_metafile: Default::default(),
            generated_page_base_path: "/".to_string(),
            generate_sitemap: false,
            is_watching: false,
            rhai_template_renderer,
            source_filesystem,
        })
        .await
    }

    #[tokio::test]
    async fn reports_document_with_changed_content_as_changed() -> Result<()> {
        let previous: BuildProjectResult = build_stub("original body").await?.into();
        let changed = build_stub("modified body")
            .await?
            .changed_compared_to(previous);

        assert_eq!(changed.changed_since_last_build.len(), 1);

        Ok(())
    }

    #[tokio::test]
    async fn reports_no_change_for_identical_content() -> Result<()> {
        let previous: BuildProjectResult = build_stub("same body").await?.into();
        let changed = build_stub("same body").await?.changed_compared_to(previous);

        assert!(changed.changed_since_last_build.is_empty());

        Ok(())
    }
}
