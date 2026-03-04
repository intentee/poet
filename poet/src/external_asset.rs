use esbuild_metafile::renders_path::RendersPath;
use rhai_components::escape_html_attribute::escape_html_attribute;

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum ExternalAsset {
    Script(String),
    Stylesheet(String),
}

impl ExternalAsset {
    pub fn render<TRendersPath: RendersPath>(&self, renders_path: &TRendersPath) -> String {
        match self {
            ExternalAsset::Script(url) => {
                let escaped_url = escape_html_attribute(&renders_path.render_path(url));

                format!("<script src=\"{escaped_url}\" async defer></script>")
            }
            ExternalAsset::Stylesheet(url) => {
                let escaped_url = escape_html_attribute(&renders_path.render_path(url));

                format!("<link rel=\"stylesheet\" href=\"{escaped_url}\">")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asset_path_renderer::AssetPathRenderer;

    fn create_path_renderer() -> AssetPathRenderer {
        AssetPathRenderer {
            base_path: "/".to_string(),
        }
    }

    #[test]
    fn script_renders_async_defer_tag() {
        let path_renderer = create_path_renderer();

        let asset = ExternalAsset::Script(
            "https://challenges.cloudflare.com/turnstile/v0/api.js".to_string(),
        );

        assert_eq!(
            asset.render(&path_renderer),
            "<script src=\"https://challenges.cloudflare.com/turnstile/v0/api.js\" async defer></script>",
        );
    }

    #[test]
    fn script_escapes_url_in_attribute() {
        let path_renderer = create_path_renderer();

        let asset =
            ExternalAsset::Script("https://example.com/script.js?foo=1&bar=\"test\"".to_string());

        assert_eq!(
            asset.render(&path_renderer),
            "<script src=\"https://example.com/script.js?foo=1&bar=&quot;test&quot;\" async defer></script>",
        );
    }

    #[test]
    fn stylesheet_renders_link_tag() {
        let path_renderer = create_path_renderer();

        let asset =
            ExternalAsset::Stylesheet("https://fonts.googleapis.com/css2?family=Inter".to_string());

        assert_eq!(
            asset.render(&path_renderer),
            "<link rel=\"stylesheet\" href=\"https://fonts.googleapis.com/css2?family=Inter\">",
        );
    }
}
