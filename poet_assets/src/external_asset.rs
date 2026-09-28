use esbuild_metafile::renders_path::RendersPath;
use rhai_components::escape_html_attribute::escape_html_attribute;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ExternalAsset {
    Script(String),
    Stylesheet(String),
}

impl ExternalAsset {
    pub fn render<TRendersPath: RendersPath>(&self, renders_path: &TRendersPath) -> String {
        match self {
            Self::Script(url) => {
                let escaped_url = escape_html_attribute(&renders_path.render_path(url));

                format!("<script src=\"{escaped_url}\" async defer></script>")
            }
            Self::Stylesheet(url) => {
                let escaped_url = escape_html_attribute(&renders_path.render_path(url));

                format!("<link rel=\"stylesheet\" href=\"{escaped_url}\">")
            }
        }
    }
}
