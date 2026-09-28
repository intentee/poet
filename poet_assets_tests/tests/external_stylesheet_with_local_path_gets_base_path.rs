use poet_assets::asset_path_renderer::AssetPathRenderer;
use poet_assets::external_asset::ExternalAsset;

#[test]
fn external_stylesheet_with_local_path_gets_base_path() {
    assert_eq!(
        ExternalAsset::Stylesheet(r"css/site.css".to_owned()).render(&AssetPathRenderer {
            base_path: "/".to_owned(),
        }),
        r#"<link rel="stylesheet" href="/css/site.css">"#
    );
}
