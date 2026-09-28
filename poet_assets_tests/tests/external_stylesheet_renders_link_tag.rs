use poet_assets::asset_path_renderer::AssetPathRenderer;
use poet_assets::external_asset::ExternalAsset;

#[test]
fn external_stylesheet_renders_link_tag() {
    assert_eq!(
        ExternalAsset::Stylesheet(r"https://fonts.googleapis.com/css2?family=Inter".to_owned())
            .render(&AssetPathRenderer {
                base_path: "/".to_owned(),
            }),
        r#"<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Inter">"#
    );
}
