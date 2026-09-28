use std::path::Path;

use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use poet_filesystem::create_parent_directories::create_parent_directories;
use tokio::fs::copy;

use crate::asset_error::AssetError;

pub async fn copy_esbuild_metafile_assets_to(
    esbuild_metafile: &EsbuildMetafile,
    source_directory: &Path,
    output_directory: &Path,
) -> Result<(), AssetError> {
    for asset_path in esbuild_metafile.get_output_paths() {
        let source_path = source_directory.join(&asset_path);
        let target_path = output_directory.join(&asset_path);

        create_parent_directories(&target_path)
            .await
            .map_err(|source| AssetError::CreateAssetDirectory {
                target_path: target_path.clone(),
                source,
            })?;
        copy(&source_path, &target_path)
            .await
            .map_err(|source| AssetError::CopyAsset {
                source_path,
                target_path,
                source,
            })?;
    }

    Ok(())
}
