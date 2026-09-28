use crate::prompt_rendering_context::PromptRenderingContext;

pub struct BuildPromptDocumentControllerCollectionParams<'params, TFilesystem> {
    pub rendering_context: PromptRenderingContext,
    pub source_filesystem: &'params TFilesystem,
}
