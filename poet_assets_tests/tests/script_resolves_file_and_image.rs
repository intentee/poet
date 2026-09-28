use poet_assets_tests::asset_script_engine::asset_script_engine;
use poet_assets_tests::fixture_asset_manager::fixture_asset_manager;
use poet_assets_tests::poet_assets_tests_error::PoetAssetsTestsError;
use rhai::Scope;

#[test]
fn script_resolves_file_and_image() -> Result<(), PoetAssetsTestsError> {
    let mut scope = Scope::new();

    scope.push(
        "assets",
        fixture_asset_manager(include_str!("../fixtures/entry_with_static_import.json"))?,
    );

    assert_eq!(
        asset_script_engine().eval_with_scope::<String>(
            &mut scope,
            r#"assets.file("logo.png") + " " + assets.image("logo.png")"#
        )?,
        r"/static/logo_ABCDEF12.png /static/logo_ABCDEF12.png"
    );

    Ok(())
}
