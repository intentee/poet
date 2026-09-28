use poet_assets::register_asset_rhai_types::register_asset_rhai_types;
use rhai::Engine;

use crate::prompt_document_argument_with_input::PromptDocumentArgumentWithInput;
use crate::prompt_document_component_context::PromptDocumentComponentContext;
use crate::prompt_document_front_matter::PromptDocumentFrontMatter;

pub fn register_prompt_rhai_types(engine: &mut Engine) {
    register_asset_rhai_types(engine);
    engine.build_type::<PromptDocumentArgumentWithInput>();
    engine.build_type::<PromptDocumentComponentContext>();
    engine.build_type::<PromptDocumentFrontMatter>();
}
