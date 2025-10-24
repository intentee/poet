use std::collections::BTreeMap;
use std::sync::Arc;

use dashmap::DashMap;

use crate::mcp::list_resources_cursor::ListResourcesCursor;
use crate::mcp::prompt::Prompt;
use crate::mcp::prompt_controller::PromptController;

#[derive(Default)]
pub struct PromptControllerCollection(pub BTreeMap<String, Arc<dyn PromptController>>);

impl PromptControllerCollection {
    pub fn list_mcp_prompts(
        &self,
        ListResourcesCursor { offset, per_page }: ListResourcesCursor,
    ) -> Vec<Prompt> {
        self.0
            .iter()
            .skip(offset)
            .take(per_page)
            .map(|(_, prompt_document_controller)| prompt_document_controller.get_mcp_prompt())
            .collect()
    }
}

impl From<DashMap<String, Arc<dyn PromptController>>> for PromptControllerCollection {
    fn from(prompt_controller_dashmap: DashMap<String, Arc<dyn PromptController>>) -> Self {
        Self(prompt_controller_dashmap.into_iter().collect())
    }
}
