use poet_assets_tests::fixture_asset_manager::fixture_asset_manager;
use poet_assets_tests::poet_assets_tests_error::PoetAssetsTestsError;

#[test]
fn image_picks_single_image_among_static_files() -> Result<(), PoetAssetsTestsError> {
    assert_eq!(
        fixture_asset_manager(include_str!("../fixtures/multiple_static_files.json"))?
            .image("img.png")?,
        "/static/img_ABCDEF12.png"
    );

    Ok(())
}
