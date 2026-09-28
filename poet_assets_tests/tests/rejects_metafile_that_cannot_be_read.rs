use std::fs::write;

use poet_assets::asset_error::AssetError;
use poet_assets::read_esbuild_metafile_or_default::read_esbuild_metafile_or_default;
use poet_assets_tests::poet_assets_tests_error::PoetAssetsTestsError;
use poet_filesystem::storage::Storage;
use tempfile::tempdir;

#[tokio::test]
async fn rejects_metafile_that_cannot_be_read() -> Result<(), PoetAssetsTestsError> {
    let directory = tempdir()?;
    let regular_file = directory.path().join("regular-file");

    write(&regular_file, "not a directory")?;

    assert!(matches!(
        read_esbuild_metafile_or_default(&Storage {
            base_directory: regular_file,
        })
        .await,
        Err(AssetError::ReadEsbuildMetafile { .. })
    ));

    Ok(())
}
