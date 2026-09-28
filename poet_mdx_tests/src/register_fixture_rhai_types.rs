use rhai::Engine;

use crate::fixture_component_context::FixtureComponentContext;

pub fn register_fixture_rhai_types(engine: &mut Engine) {
    engine.build_type::<FixtureComponentContext>();
}
