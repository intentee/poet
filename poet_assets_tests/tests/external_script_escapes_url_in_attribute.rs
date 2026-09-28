use poet_assets::asset_path_renderer::AssetPathRenderer;
use poet_assets::external_asset::ExternalAsset;

#[test]
fn external_script_escapes_url_in_attribute() {
    assert_eq!(
        ExternalAsset::Script(r#"https://example.com/script.js?foo=1&bar="test""#.to_owned())
            .render(&AssetPathRenderer {
                base_path: "/".to_owned(),
            }),
        r#"<script src="https://example.com/script.js?foo=1&bar=&quot;test&quot;" async defer></script>"#
    );
}
