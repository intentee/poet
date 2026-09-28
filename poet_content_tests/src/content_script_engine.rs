use poet_content::register_content_rhai_types::register_content_rhai_types;
use rhai::Engine;

#[must_use]
pub fn content_script_engine() -> Engine {
    let mut engine = Engine::new();

    register_content_rhai_types(&mut engine);

    engine
}
