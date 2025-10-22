use std::collections::BTreeMap;

use dashmap::DashMap;

use crate::mcp::list_resources_cursor::ListResourcesCursor;
use crate::mcp::prompt::Prompt;
use crate::prompt_controller::PromptController;

#[derive(Default)]
pub struct PromptControllerCollection(pub BTreeMap<String, PromptController>);

impl PromptControllerCollection {
    pub fn list_mcp_prompts(
        &self,
        ListResourcesCursor { offset, per_page }: ListResourcesCursor,
    ) -> Vec<Prompt> {
        self.0
            .iter()
            .skip(offset)
            .take(per_page)
            .map(|(_, prompt_controller)| prompt_controller.get_mcp_prompt())
            .collect()
    }
}

impl From<DashMap<String, PromptController>> for PromptControllerCollection {
    fn from(prompt_controller_dashmap: DashMap<String, PromptController>) -> Self {
        Self(prompt_controller_dashmap.into_iter().collect())
    }
}
