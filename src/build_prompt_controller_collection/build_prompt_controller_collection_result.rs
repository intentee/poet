use dashmap::DashMap;

use crate::prompt_controller::PromptController;

pub struct BuildPromptControllerCollectionResult {
    pub prompt_controller_collection: DashMap<String, PromptController>,
}
