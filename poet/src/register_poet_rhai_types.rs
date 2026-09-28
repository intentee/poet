use poet_content::register_content_rhai_types::register_content_rhai_types;
use poet_prompt::register_prompt_rhai_types::register_prompt_rhai_types;
use rhai::Engine;

pub fn register_poet_rhai_types(engine: &mut Engine) {
    register_content_rhai_types(engine);
    register_prompt_rhai_types(engine);
}
