use rhai::Engine;

use crate::asset_manager::AssetManager;

pub fn register_asset_rhai_types(engine: &mut Engine) {
    engine.build_type::<AssetManager>();
}
