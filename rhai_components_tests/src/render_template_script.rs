use rhai::EvalAltResult;
use rhai::Func;

use crate::dummy_context::DummyContext;
use crate::fixtures_engine::fixtures_engine;

pub fn render_template_script(
    template_script: &str,
    dummy_context: DummyContext,
) -> Result<String, Box<EvalAltResult>> {
    Func::<(DummyContext,), String>::create_from_script(
        fixtures_engine(),
        template_script,
        "template",
    )?(dummy_context)
}
