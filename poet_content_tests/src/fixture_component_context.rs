use std::str::FromStr as _;
use std::sync::Arc;

use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use poet_assets::asset_manager::AssetManager;
use poet_assets::asset_path_renderer::AssetPathRenderer;
use poet_content::author::Author;
use poet_content::content_document_component_context::ContentDocumentComponentContext;
use poet_content::content_document_context::ContentDocumentContext;
use poet_content::content_document_reference::ContentDocumentReference;
use poet_content::content_site_context::ContentSiteContext;
use poet_content::table_of_contents_state::TableOfContentsState;

use crate::poet_content_tests_error::PoetContentTestsError;

pub fn fixture_component_context(
    authors: Vec<Author>,
    reference: ContentDocumentReference,
    site: ContentSiteContext,
) -> Result<ContentDocumentComponentContext, PoetContentTestsError> {
    Ok(ContentDocumentComponentContext {
        asset_manager: AssetManager::from_esbuild_metafile(
            Arc::new(EsbuildMetafile::from_str(include_str!(
                "../fixtures/metafile.json"
            ))?),
            AssetPathRenderer {
                base_path: "/".to_owned(),
            },
        ),
        document: ContentDocumentContext { authors, reference },
        site,
        table_of_contents: TableOfContentsState::BeingCollected,
    })
}
