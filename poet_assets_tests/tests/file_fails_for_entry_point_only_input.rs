use poet_assets::asset_error::AssetError;
use poet_assets_tests::fixture_asset_manager::fixture_asset_manager;
use poet_assets_tests::poet_assets_tests_error::PoetAssetsTestsError;

#[test]
fn file_fails_for_entry_point_only_input() -> Result<(), PoetAssetsTestsError> {
    assert!(matches!(
        fixture_asset_manager(include_str!("../fixtures/entry_with_static_import.json"))?.file("app.ts"),
        Err(AssetError::AssetHasNoStaticFile { input_path }) if input_path == "app.ts"
    ));

    Ok(())
}
