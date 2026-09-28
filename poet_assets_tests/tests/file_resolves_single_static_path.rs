use poet_assets_tests::fixture_asset_manager::fixture_asset_manager;
use poet_assets_tests::poet_assets_tests_error::PoetAssetsTestsError;

#[test]
fn file_resolves_single_static_path() -> Result<(), PoetAssetsTestsError> {
    assert_eq!(
        fixture_asset_manager(include_str!("../fixtures/entry_with_static_import.json"))?
            .file("logo.png")?,
        "/static/logo_ABCDEF12.png"
    );

    Ok(())
}
