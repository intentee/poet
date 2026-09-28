use poet_assets::read_esbuild_metafile_or_default::read_esbuild_metafile_or_default;
use poet_assets_tests::poet_assets_tests_error::PoetAssetsTestsError;
use poet_filesystem::memory::Memory;

#[tokio::test]
async fn falls_back_to_empty_metafile_when_it_is_missing() -> Result<(), PoetAssetsTestsError> {
    assert!(
        read_esbuild_metafile_or_default(&Memory::default())
            .await?
            .get_output_paths()
            .is_empty()
    );

    Ok(())
}
