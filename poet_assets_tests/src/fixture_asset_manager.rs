use std::str::FromStr as _;
use std::sync::Arc;

use esbuild_metafile::error::Error as EsbuildMetafileError;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use poet_assets::asset_manager::AssetManager;
use poet_assets::asset_path_renderer::AssetPathRenderer;

pub fn fixture_asset_manager(metafile_json: &str) -> Result<AssetManager, EsbuildMetafileError> {
    Ok(AssetManager::from_esbuild_metafile(
        Arc::new(EsbuildMetafile::from_str(metafile_json)?),
        AssetPathRenderer {
            base_path: "/".to_owned(),
        },
    ))
}
