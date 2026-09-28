use poet_assets::asset_error::AssetError;
use poet_assets_tests::fixture_asset_manager::fixture_asset_manager;
use poet_assets_tests::poet_assets_tests_error::PoetAssetsTestsError;

#[test]
fn file_fails_for_input_with_multiple_static_files() -> Result<(), PoetAssetsTestsError> {
    assert!(matches!(
        fixture_asset_manager(include_str!("../fixtures/multiple_static_files.json"))?.file("img.png"),
        Err(AssetError::AssetHasMultipleStaticFiles { input_path }) if input_path == "img.png"
    ));

    Ok(())
}
