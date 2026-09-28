use rhai::Engine;

use crate::prompt_document_component_context::PromptDocumentComponentContext;
use crate::prompt_document_front_matter::PromptDocumentFrontMatter;
use crate::prompt_document_front_matter::argument_with_input::ArgumentWithInput;

pub fn register_prompt_rhai_types(engine: &mut Engine) {
    engine.build_type::<ArgumentWithInput>();
    engine.build_type::<PromptDocumentComponentContext>();
    engine.build_type::<PromptDocumentFrontMatter>();
}
