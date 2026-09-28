use std::sync::Arc;

use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use esbuild_metafile::input_lookup::InputLookup;
use esbuild_metafile::input_properties::InputProperties;

use crate::asset_error::AssetError;
use crate::asset_input::AssetInput;

#[derive(Clone)]
pub struct AssetInputResolver {
    pub esbuild_metafile: Arc<EsbuildMetafile>,
}

impl AssetInputResolver {
    pub fn resolve(&self, input_path: &str) -> Result<AssetInput, AssetError> {
        match self.esbuild_metafile.input(input_path) {
            InputLookup::Found(InputProperties {
                outputs,
                static_paths,
            }) => Ok(AssetInput {
                input_path: input_path.to_owned(),
                outputs,
                static_paths,
            }),
            InputLookup::NotFound => Err(AssetError::AssetNotFound {
                input_path: input_path.to_owned(),
            }),
        }
    }
}
