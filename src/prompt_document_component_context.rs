use crate::asset_manager::AssetManager;
use crate::content_document_linker::ContentDocumentLinker;

#[derive(Clone)]
pub struct PromptDocumentComponentContext {
    pub asset_manager: AssetManager,
    pub content_document_linker: ContentDocumentLinker,
}
