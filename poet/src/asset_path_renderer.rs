use esbuild_metafile::renders_path::RendersPath;

use crate::is_external_link::is_external_link;

#[derive(Clone)]
pub struct AssetPathRenderer {
    pub base_path: String,
}

impl RendersPath for AssetPathRenderer {
    fn render_path(&self, path: &str) -> String {
        if is_external_link(path) {
            path.to_string()
        } else {
            format!("{}{path}", self.base_path)
        }
    }
}
