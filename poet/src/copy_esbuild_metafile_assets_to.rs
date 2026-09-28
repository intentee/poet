use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use tokio::fs;

use crate::filesystem::storage::create_parent_directories::create_parent_directories;

pub async fn copy_esbuild_metafile_assets_to(
    esbuild_metafile: Arc<EsbuildMetafile>,
    source_directory: &Path,
    output_directory: &Path,
) -> Result<()> {
    for asset_path in esbuild_metafile.get_output_paths().iter() {
        let target_path = output_directory.join(asset_path);

        create_parent_directories(&target_path).await?;

        fs::copy(source_directory.join(asset_path), target_path).await?;
    }

    Ok(())
}
