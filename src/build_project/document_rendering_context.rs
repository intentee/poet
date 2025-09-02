use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use esbuild_metafile::EsbuildMetaFile;
use syntect::parsing::SyntaxSet;

use crate::asset_path_renderer::AssetPathRenderer;
use crate::markdown_document::MarkdownDocument;
use crate::markdown_document_reference::MarkdownDocumentReference;
use crate::rhai_markdown_document_collection::RhaiMarkdownDocumentCollection;
use crate::rhai_template_renderer::RhaiTemplateRenderer;

pub struct DocumentRenderingContext<'render> {
    pub asset_path_renderer: AssetPathRenderer,
    pub available_collections: Arc<HashSet<String>>,
    pub esbuild_metafile: Arc<EsbuildMetaFile>,
    pub is_watching: bool,
    pub markdown_basename_by_id: Arc<HashMap<String, String>>,
    pub markdown_document: &'render MarkdownDocument,
    pub markdown_document_by_basename: Arc<HashMap<String, MarkdownDocumentReference>>,
    pub rhai_markdown_document_collections: Arc<HashMap<String, RhaiMarkdownDocumentCollection>>,
    pub rhai_template_renderer: &'render RhaiTemplateRenderer,
    pub syntax_set: &'render SyntaxSet,
}
