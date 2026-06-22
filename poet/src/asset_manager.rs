use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::Mutex;

use esbuild_metafile::EsbuildMetaFile;
use esbuild_metafile::HttpPreloader;
use esbuild_metafile::renders_path::RendersPath;
use rhai::CustomType;
use rhai::EvalAltResult;
use rhai::TypeBuilder;

use crate::asset_path_renderer::AssetPathRenderer;
use crate::external_asset::ExternalAsset;

#[derive(Clone)]
pub struct AssetManager {
    esbuild_metafile: Arc<EsbuildMetaFile>,
    external_assets: Arc<Mutex<BTreeSet<ExternalAsset>>>,
    http_preloader: Arc<HttpPreloader>,
    path_renderer: AssetPathRenderer,
}

impl AssetManager {
    pub fn from_esbuild_metafile(
        esbuild_metafile: Arc<EsbuildMetaFile>,
        path_renderer: AssetPathRenderer,
    ) -> Self {
        AssetManager {
            esbuild_metafile: esbuild_metafile.clone(),
            external_assets: Arc::new(Mutex::new(BTreeSet::new())),
            http_preloader: Arc::new(HttpPreloader::new(esbuild_metafile)),
            path_renderer,
        }
    }

    pub fn file(&self, asset: &str) -> Result<String, String> {
        if let Some(static_paths) = self.esbuild_metafile.find_static_paths_for_input(asset) {
            if static_paths.len() != 1 {
                return Err("Unexpectedly multiple assets resolved to the same input".into());
            }

            if let Some(path) = static_paths.first() {
                return Ok(self.path_renderer.render_path(path));
            }
        }

        Err(format!("Asset not found: '{asset}'"))
    }

    fn rhai_add(&mut self, asset: String) -> Result<(), Box<EvalAltResult>> {
        if self.http_preloader.register_input(&asset).is_none() {
            return Err(format!("Asset not found: {asset}").into());
        }

        Ok(())
    }

    fn rhai_file(&mut self, asset: String) -> Result<String, Box<EvalAltResult>> {
        Ok(self.file(&asset)?)
    }

    fn rhai_preload(&mut self, asset: String) {
        self.http_preloader.register_preload(&asset);
    }

    fn rhai_render(&mut self) -> String {
        let mut rendered_assets: String = String::new();
        let mut rendered_preloads: BTreeSet<String> = BTreeSet::new();
        let mut rendered_includes: BTreeSet<String> = BTreeSet::new();

        for path in self.http_preloader.preloads.iter() {
            rendered_preloads.insert(path.render(&self.path_renderer));
        }

        for path in self.http_preloader.includes.iter() {
            rendered_includes.insert(path.render(&self.path_renderer));
        }

        for external_asset in self
            .external_assets
            .lock()
            .expect("external assets mutex poisoned")
            .iter()
        {
            rendered_includes.insert(external_asset.render(&self.path_renderer));
        }

        for element in rendered_preloads {
            rendered_assets.push_str(&element);
        }

        for element in rendered_includes {
            rendered_assets.push_str(&element);
        }

        rendered_assets
    }

    fn rhai_script(&mut self, url: String) {
        self.external_assets
            .lock()
            .expect("external assets mutex poisoned")
            .insert(ExternalAsset::Script(url));
    }

    fn rhai_stylesheet(&mut self, url: String) {
        self.external_assets
            .lock()
            .expect("external assets mutex poisoned")
            .insert(ExternalAsset::Stylesheet(url));
    }
}

impl CustomType for AssetManager {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("AssetManager")
            .with_fn("add", Self::rhai_add)
            .with_fn("file", Self::rhai_file)
            .with_fn("preload", Self::rhai_preload)
            .with_fn("render", Self::rhai_render)
            .with_fn("script", Self::rhai_script)
            .with_fn("stylesheet", Self::rhai_stylesheet);
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use indoc::indoc;

    use super::*;
    use crate::asset_path_renderer::AssetPathRenderer;

    fn asset_manager(metafile_json: &str) -> Result<AssetManager, anyhow::Error> {
        Ok(AssetManager::from_esbuild_metafile(
            Arc::new(EsbuildMetaFile::from_str(metafile_json)?),
            AssetPathRenderer {
                base_path: "/".to_string(),
            },
        ))
    }

    #[test]
    fn file_resolves_single_static_path() -> Result<(), anyhow::Error> {
        let metafile = indoc! {r#"
            {
                "outputs": {
                    "static/logo_ABCDEF12.png": {
                        "imports": [],
                        "inputs": { "logo.png": {} }
                    },
                    "static/entry_ABCDEF12.js": {
                        "imports": [{ "path": "static/logo_ABCDEF12.png" }],
                        "entryPoint": "logo.png",
                        "inputs": {}
                    }
                }
            }
        "#};

        assert_eq!(
            asset_manager(metafile)?.file("logo.png"),
            Ok("/static/logo_ABCDEF12.png".to_string())
        );

        Ok(())
    }

    #[test]
    fn file_fails_for_unknown_input() -> Result<(), anyhow::Error> {
        assert_eq!(
            asset_manager(r#"{ "outputs": {} }"#)?.file("missing.png"),
            Err("Asset not found: 'missing.png'".to_string())
        );

        Ok(())
    }

    #[test]
    fn file_fails_when_input_resolves_to_multiple_paths() -> Result<(), anyhow::Error> {
        let metafile = indoc! {r#"
            {
                "outputs": {
                    "static/a_AAAAAAAA.png": {
                        "imports": [],
                        "inputs": { "img.png": {} }
                    },
                    "static/b_BBBBBBBB.png": {
                        "imports": [],
                        "inputs": { "img.png": {} }
                    },
                    "static/entry_CCCCCCCC.js": {
                        "imports": [
                            { "path": "static/a_AAAAAAAA.png" },
                            { "path": "static/b_BBBBBBBB.png" }
                        ],
                        "entryPoint": "img.png",
                        "inputs": {}
                    }
                }
            }
        "#};

        assert!(asset_manager(metafile)?.file("img.png").is_err());

        Ok(())
    }

    const ENTRY_METAFILE: &str = indoc! {r#"
        {
            "outputs": {
                "static/logo_ABCDEF12.png": {
                    "imports": [],
                    "inputs": { "logo.png": {} }
                },
                "static/entry_ABCDEF12.js": {
                    "imports": [{ "path": "static/logo_ABCDEF12.png" }],
                    "entryPoint": "logo.png",
                    "inputs": {}
                }
            }
        }
    "#};

    #[test]
    fn render_emits_tag_for_external_script() -> Result<(), anyhow::Error> {
        let mut manager = asset_manager(r#"{ "outputs": {} }"#)?;

        manager.rhai_script("https://example.com/app.js".to_string());

        assert!(
            manager
                .rhai_render()
                .contains("<script src=\"https://example.com/app.js\" async defer></script>")
        );

        Ok(())
    }

    #[test]
    fn render_emits_tag_for_external_stylesheet() -> Result<(), anyhow::Error> {
        let mut manager = asset_manager(r#"{ "outputs": {} }"#)?;

        manager.rhai_stylesheet("https://example.com/app.css".to_string());

        assert!(
            manager
                .rhai_render()
                .contains("<link rel=\"stylesheet\" href=\"https://example.com/app.css\">")
        );

        Ok(())
    }

    #[test]
    fn add_registers_known_input_for_rendering() -> Result<(), anyhow::Error> {
        let mut manager = asset_manager(ENTRY_METAFILE)?;

        assert!(manager.rhai_add("logo.png".to_string()).is_ok());
        assert!(manager.rhai_render().contains("/static/entry_ABCDEF12.js"));

        Ok(())
    }

    #[test]
    fn add_fails_for_unknown_input() -> Result<(), anyhow::Error> {
        let mut manager = asset_manager(r#"{ "outputs": {} }"#)?;

        assert!(manager.rhai_add("missing.png".to_string()).is_err());

        Ok(())
    }

    #[test]
    fn preload_registers_known_input_for_rendering() -> Result<(), anyhow::Error> {
        let mut manager = asset_manager(ENTRY_METAFILE)?;

        manager.rhai_preload("logo.png".to_string());

        assert!(manager.rhai_render().contains("/static/"));

        Ok(())
    }
}
