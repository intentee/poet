use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;

use log::info;
use poet_filesystem::filesystem::Filesystem;
use poet_mdx::build_timer::BuildTimer;
use sitemap_rs::jiff::Timestamp;
use sitemap_rs::jiff::tz::TimeZone;

use crate::build_project_params::BuildProjectParams;
use crate::build_project_result_stub::BuildProjectResultStub;
use crate::content_document_basename::ContentDocumentBasename;
use crate::content_document_index::ContentDocumentIndex;
use crate::content_document_linker::ContentDocumentLinker;
use crate::content_document_renderer::ContentDocumentRenderer;
use crate::content_document_source::ContentDocumentSource;
use crate::content_error::ContentError;
use crate::content_site_context::ContentSiteContext;
use crate::create_sitemap::create_sitemap;
use crate::default_syntax_set::DEFAULT_SYNTAX_SET;
use crate::load_content_document_sources::load_content_document_sources;

pub async fn build_project<TFilesystem: Filesystem>(
    BuildProjectParams {
        asset_path_renderer,
        authors,
        esbuild_metafile,
        generate_sitemap,
        generated_page_base_path,
        is_watching,
        rhai_template_renderer,
        source_filesystem,
    }: BuildProjectParams<'_, TFilesystem>,
) -> Result<BuildProjectResultStub, ContentError> {
    info!("Processing content files...");

    let _build_timer = BuildTimer::default();
    let content_document_sources =
        load_content_document_sources(source_filesystem, &generated_page_base_path).await?;
    let content_document_index = ContentDocumentIndex::build(&content_document_sources)?;
    let content_document_collections_ranked = content_document_index.rank_collections()?;
    let site_context = ContentSiteContext {
        available_authors: Arc::new(authors),
        content_document_collections_ranked: Arc::new(content_document_collections_ranked),
        content_document_linker: ContentDocumentLinker {
            content_document_basename_by_id: Arc::new(
                content_document_index.content_document_basename_by_id,
            ),
            content_document_by_basename: Arc::new(
                content_document_index.content_document_by_basename,
            ),
        },
        is_watching,
    };
    let memory_filesystem = ContentDocumentRenderer {
        asset_path_renderer: &asset_path_renderer,
        esbuild_metafile: &esbuild_metafile,
        rhai_template_renderer: &rhai_template_renderer,
        site_context: &site_context,
        syntax_set: &DEFAULT_SYNTAX_SET,
    }
    .render_pages(&content_document_sources)?;
    let rendered_content_document_sources: BTreeMap<
        ContentDocumentBasename,
        ContentDocumentSource,
    > = content_document_sources
        .into_iter()
        .filter(|content_document_source| content_document_source.reference.front_matter.render)
        .map(|content_document_source| {
            (
                content_document_source.reference.basename(),
                content_document_source,
            )
        })
        .collect();

    generate_sitemap
        .then(|| {
            create_sitemap(
                &Timestamp::now().to_zoned(TimeZone::UTC),
                rendered_content_document_sources
                    .values()
                    .map(|content_document_source| &content_document_source.reference),
            )
        })
        .transpose()
        .map(|sitemap| {
            if let Some(sitemap) = sitemap {
                memory_filesystem.set_file_contents_sync(Path::new("sitemap.xml"), &sitemap);
            }

            BuildProjectResultStub {
                content_document_linker: site_context.content_document_linker,
                content_document_sources: Arc::new(rendered_content_document_sources),
                esbuild_metafile,
                memory_filesystem: Arc::new(memory_filesystem),
            }
        })
}
