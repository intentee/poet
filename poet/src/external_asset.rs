use esbuild_metafile::renders_path::RendersPath;

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum ExternalAsset {
    Script(String),
    Stylesheet(String),
}

impl ExternalAsset {
    pub fn render<TRendersPath: RendersPath>(&self, renders_path: &TRendersPath) -> String {
        match self {
            ExternalAsset::Script(url) => {
                format!(
                    "<script src=\"{}\" async defer></script>",
                    renders_path.render_path(url),
                )
            }
            ExternalAsset::Stylesheet(url) => {
                format!(
                    "<link rel=\"stylesheet\" href=\"{}\">",
                    renders_path.render_path(url),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asset_path_renderer::AssetPathRenderer;

    #[test]
    fn script_renders_async_defer_tag() {
        let path_renderer = AssetPathRenderer {
            base_path: "/".to_string(),
        };

        let asset = ExternalAsset::Script(
            "https://challenges.cloudflare.com/turnstile/v0/api.js".to_string(),
        );

        assert_eq!(
            asset.render(&path_renderer),
            "<script src=\"https://challenges.cloudflare.com/turnstile/v0/api.js\" async defer></script>",
        );
    }

    #[test]
    fn stylesheet_renders_link_tag() {
        let path_renderer = AssetPathRenderer {
            base_path: "/".to_string(),
        };

        let asset =
            ExternalAsset::Stylesheet("https://fonts.googleapis.com/css2?family=Inter".to_string());

        assert_eq!(
            asset.render(&path_renderer),
            "<link rel=\"stylesheet\" href=\"https://fonts.googleapis.com/css2?family=Inter\">",
        );
    }
}
