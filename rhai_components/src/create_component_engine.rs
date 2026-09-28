use rhai::Dynamic;
use rhai::Engine;

use crate::component_nesting_depth::ComponentNestingDepth;
use crate::component_syntax::evaluate_component::evaluate_component;
use crate::component_syntax::parse_component::parse_component;
use crate::rhai_helpers::clsx::clsx;
use crate::rhai_helpers::error::error;
use crate::rhai_helpers::has::has;

pub fn create_component_engine() -> Engine {
    let mut engine = Engine::new();

    engine.set_default_tag(Dynamic::from(ComponentNestingDepth::default()));
    engine.set_fail_on_invalid_map_property(true);
    engine.set_max_call_levels(128);
    engine.set_max_expr_depths(256, 256);

    engine.register_fn("clsx", clsx);
    engine.register_fn("error", error);
    engine.register_fn("has", has);

    engine.register_custom_syntax_without_look_ahead_raw(
        "component",
        parse_component,
        false,
        evaluate_component,
    );

    engine
}
