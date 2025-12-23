use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use esbuild_metafile::EsbuildMetaFile;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;
use syntect::parsing::SyntaxSet;

use crate::asset_path_renderer::AssetPathRenderer;
use crate::content_document::ContentDocument;
use crate::content_document_collection_ranked::ContentDocumentCollectionRanked;
use crate::content_document_linker::ContentDocumentLinker;

pub struct ContentDocumentRenderingContext<'render> {
    pub asset_path_renderer: AssetPathRenderer,
    pub available_collections: Arc<HashSet<String>>,
    pub content_document: &'render ContentDocument,
    pub content_document_collections_ranked: Arc<HashMap<String, ContentDocumentCollectionRanked>>,
    pub content_document_linker: ContentDocumentLinker,
    pub esbuild_metafile: Arc<EsbuildMetaFile>,
    pub is_watching: bool,
    pub rhai_template_renderer: &'render RhaiTemplateRenderer,
    pub syntax_set: &'render SyntaxSet,
}
