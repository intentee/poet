use std::sync::Arc;

use poet_filesystem::source_file::SourceFile;

use crate::prompt_rendering_context::PromptRenderingContext;

pub struct BuildPromptDocumentControllerParams {
    pub rendering_context: Arc<PromptRenderingContext>,
    pub source_file: SourceFile,
}
