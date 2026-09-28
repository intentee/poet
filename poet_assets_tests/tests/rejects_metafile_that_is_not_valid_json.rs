use std::path::Path;

use poet_assets::asset_error::AssetError;
use poet_assets::esbuild_metafile_path::ESBUILD_METAFILE_PATH;
use poet_assets::read_esbuild_metafile_or_default::read_esbuild_metafile_or_default;
use poet_filesystem::memory::Memory;

#[tokio::test]
async fn rejects_metafile_that_is_not_valid_json() {
    let memory = Memory::default();

    memory.set_file_contents_sync(Path::new(ESBUILD_METAFILE_PATH), "{ not json");

    assert!(matches!(
        read_esbuild_metafile_or_default(&memory).await,
        Err(AssetError::ParseEsbuildMetafile { .. })
    ));
}
