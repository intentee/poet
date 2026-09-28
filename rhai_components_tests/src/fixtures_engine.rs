use rhai::Engine;
use rhai::module_resolvers::FileModuleResolver;
use rhai_components::create_component_engine::create_component_engine;

use crate::dummy_asset_collection::DummyAssetCollection;
use crate::dummy_context::DummyContext;

#[must_use]
pub fn fixtures_engine() -> Engine {
    let mut engine = create_component_engine();

    engine.set_module_resolver(FileModuleResolver::new_with_path(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/fixtures"
    )));
    engine.build_type::<DummyAssetCollection>();
    engine.build_type::<DummyContext>();

    engine
}
