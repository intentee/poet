use poet_assets::asset_error::AssetError;
use poet_assets_tests::asset_error_of::asset_error_of;
use poet_assets_tests::asset_script_engine::asset_script_engine;
use poet_assets_tests::fixture_asset_manager::fixture_asset_manager;
use poet_assets_tests::poet_assets_tests_error::PoetAssetsTestsError;
use rhai::Dynamic;
use rhai::Scope;

#[test]
fn script_reports_unknown_asset_in_preload() -> Result<(), PoetAssetsTestsError> {
    let mut scope = Scope::new();

    scope.push(
        "assets",
        fixture_asset_manager(include_str!("../fixtures/static_script.json"))?,
    );

    assert!(
        asset_script_engine()
            .eval_with_scope::<Dynamic>(&mut scope, r#"assets.preload("missing.ts")"#)
            .is_err_and(|eval_alt_result| matches!(
                asset_error_of(&eval_alt_result),
                Some(AssetError::AssetNotFound { input_path }) if input_path == "missing.ts"
            ))
    );

    Ok(())
}
