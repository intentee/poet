use std::collections::BTreeMap;

use poet_mcp::list_resources_cursor::ListResourcesCursor;
use poet_mcp::prompt::Prompt;

use crate::prompt_document_controller::PromptDocumentController;

pub struct PromptDocumentControllerCollection {
    pub prompt_document_controllers: BTreeMap<String, PromptDocumentController>,
}

impl PromptDocumentControllerCollection {
    #[must_use]
    pub fn list_mcp_prompts(
        &self,
        ListResourcesCursor { offset, per_page }: ListResourcesCursor,
    ) -> Vec<Prompt> {
        self.prompt_document_controllers
            .values()
            .skip(offset)
            .take(per_page)
            .map(PromptDocumentController::mcp_prompt)
            .collect()
    }
}
