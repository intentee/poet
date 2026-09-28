use esbuild_metafile::renders_path::RendersPath as _;

use crate::asset_error::AssetError;
use crate::asset_input::AssetInput;
use crate::asset_path_renderer::AssetPathRenderer;
use crate::is_image_path::is_image_path;

#[derive(Clone)]
pub struct StaticAssetResolver {
    pub path_renderer: AssetPathRenderer,
}

impl StaticAssetResolver {
    pub fn file(
        &self,
        AssetInput {
            input_path,
            static_paths,
            ..
        }: &AssetInput,
    ) -> Result<String, AssetError> {
        match static_paths.as_slice() {
            [static_path] => Ok(self.path_renderer.render_path(static_path)),
            [] => Err(AssetError::AssetHasNoStaticFile {
                input_path: input_path.clone(),
            }),
            _ => Err(AssetError::AssetHasMultipleStaticFiles {
                input_path: input_path.clone(),
            }),
        }
    }

    pub fn image(
        &self,
        AssetInput {
            input_path,
            static_paths,
            ..
        }: &AssetInput,
    ) -> Result<String, AssetError> {
        let image_paths: Vec<&String> = static_paths
            .iter()
            .filter(|static_path| is_image_path(static_path))
            .collect();

        match image_paths.as_slice() {
            [image_path] => Ok(self.path_renderer.render_path(image_path)),
            [] => Err(AssetError::AssetHasNoImage {
                input_path: input_path.clone(),
            }),
            _ => Err(AssetError::AssetHasMultipleImages {
                input_path: input_path.clone(),
            }),
        }
    }
}
