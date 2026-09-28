use std::fs::create_dir;

use poet_assets::asset_error::AssetError;
use poet_assets::esbuild_metafile_path::ESBUILD_METAFILE_PATH;
use poet_assets::read_esbuild_metafile_or_default::read_esbuild_metafile_or_default;
use poet_assets_tests::poet_assets_tests_error::PoetAssetsTestsError;
use poet_filesystem::storage::Storage;
use tempfile::tempdir;

#[tokio::test]
async fn rejects_metafile_path_that_is_a_directory() -> Result<(), PoetAssetsTestsError> {
    let source_directory = tempdir()?;

    create_dir(source_directory.path().join(ESBUILD_METAFILE_PATH))?;

    assert!(matches!(
        read_esbuild_metafile_or_default(&Storage {
            base_directory: source_directory.path().to_path_buf(),
        })
        .await,
        Err(AssetError::EsbuildMetafileIsDirectory { .. })
    ));

    Ok(())
}
