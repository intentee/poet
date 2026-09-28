use poet_assets::asset_path_renderer::AssetPathRenderer;
use poet_assets::external_asset::ExternalAsset;

#[test]
fn external_script_renders_async_defer_tag() {
    assert_eq!(
        ExternalAsset::Script(r"https://challenges.cloudflare.com/turnstile/v0/api.js".to_owned())
            .render(&AssetPathRenderer {
                base_path: "/".to_owned(),
            }),
        r#"<script src="https://challenges.cloudflare.com/turnstile/v0/api.js" async defer></script>"#
    );
}
