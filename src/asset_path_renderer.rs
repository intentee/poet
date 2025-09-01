use esbuild_metafile::renders_path::RendersPath;

#[derive(Clone)]
pub struct AssetPathRenderer {
    pub base_path: String,
}

impl RendersPath for AssetPathRenderer {
    fn render_path(&self, path: &str) -> String {
        if path.starts_with("http://") || path.starts_with("https://") {
            path.to_string()
        } else {
            format!("{}{path}", self.base_path)
        }
    }
}
