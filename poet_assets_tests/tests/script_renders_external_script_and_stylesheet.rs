use poet_assets_tests::asset_script_engine::asset_script_engine;
use poet_assets_tests::fixture_asset_manager::fixture_asset_manager;
use poet_assets_tests::poet_assets_tests_error::PoetAssetsTestsError;
use rhai::Scope;

#[test]
fn script_renders_external_script_and_stylesheet() -> Result<(), PoetAssetsTestsError> {
    let mut scope = Scope::new();

    scope.push(
        "assets",
        fixture_asset_manager(include_str!("../fixtures/static_script.json"))?,
    );

    assert_eq!(
        asset_script_engine().eval_with_scope::<String>(&mut scope, r#"assets.script("https://example.com/app.js"); assets.stylesheet("https://example.com/app.css"); assets.render()"#)?,
        r#"<link rel="stylesheet" href="https://example.com/app.css"><script src="https://example.com/app.js" async defer></script>"#
    );

    Ok(())
}
