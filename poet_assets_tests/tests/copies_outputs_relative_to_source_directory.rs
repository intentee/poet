use std::fs::create_dir;
use std::fs::read_to_string;
use std::fs::write;
use std::str::FromStr as _;

use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use poet_assets::copy_esbuild_metafile_assets_to::copy_esbuild_metafile_assets_to;
use poet_assets_tests::poet_assets_tests_error::PoetAssetsTestsError;
use tempfile::tempdir;

#[tokio::test]
async fn copies_outputs_relative_to_source_directory() -> Result<(), PoetAssetsTestsError> {
    let source_directory = tempdir()?;
    let output_directory = tempdir()?;

    create_dir(source_directory.path().join("static"))?;
    write(
        source_directory.path().join("static/data_ABCDEF12.js"),
        "console.log(1);",
    )?;

    copy_esbuild_metafile_assets_to(
        &EsbuildMetafile::from_str(include_str!("../fixtures/static_script.json"))?,
        source_directory.path(),
        output_directory.path(),
    )
    .await?;

    assert_eq!(
        read_to_string(output_directory.path().join("static/data_ABCDEF12.js"))?,
        "console.log(1);"
    );

    Ok(())
}
