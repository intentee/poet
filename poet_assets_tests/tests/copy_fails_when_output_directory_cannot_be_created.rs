use std::fs::write;
use std::str::FromStr as _;

use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use poet_assets::asset_error::AssetError;
use poet_assets::copy_esbuild_metafile_assets_to::copy_esbuild_metafile_assets_to;
use poet_assets_tests::poet_assets_tests_error::PoetAssetsTestsError;
use tempfile::tempdir;

#[tokio::test]
async fn copy_fails_when_output_directory_cannot_be_created() -> Result<(), PoetAssetsTestsError> {
    let source_directory = tempdir()?;
    let output_directory = tempdir()?;

    write(output_directory.path().join("static"), "not a directory")?;

    assert!(matches!(
        copy_esbuild_metafile_assets_to(
            &EsbuildMetafile::from_str(include_str!("../fixtures/static_script.json"))?,
            source_directory.path(),
            output_directory.path(),
        )
        .await,
        Err(AssetError::CreateAssetDirectory { .. })
    ));

    Ok(())
}
