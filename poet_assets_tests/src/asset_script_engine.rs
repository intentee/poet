use poet_assets::asset_manager::AssetManager;
use rhai::Engine;

#[must_use]
pub fn asset_script_engine() -> Engine {
    let mut engine = Engine::new();

    engine.build_type::<AssetManager>();

    engine
}
