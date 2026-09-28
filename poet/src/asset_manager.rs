use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::Mutex;

use dashmap::DashSet;
use esbuild_metafile::asset::Asset;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use esbuild_metafile::input_lookup::InputLookup;
use esbuild_metafile::input_properties::InputProperties;
use esbuild_metafile::output_lookup::OutputLookup;
use esbuild_metafile::output_properties::OutputProperties;
use esbuild_metafile::preloadable_asset::PreloadableAsset;
use esbuild_metafile::renders_path::RendersPath;
use rhai::CustomType;
use rhai::EvalAltResult;
use rhai::TypeBuilder;

use crate::asset_path_renderer::AssetPathRenderer;
use crate::external_asset::ExternalAsset;
use crate::is_image_path::is_image_path;

#[derive(Clone)]
pub struct AssetManager {
    esbuild_metafile: Arc<EsbuildMetafile>,
    external_assets: Arc<Mutex<BTreeSet<ExternalAsset>>>,
    includes: Arc<DashSet<Asset>>,
    path_renderer: AssetPathRenderer,
    preloads: Arc<DashSet<PreloadableAsset>>,
}

impl AssetManager {
    pub fn from_esbuild_metafile(
        esbuild_metafile: Arc<EsbuildMetafile>,
        path_renderer: AssetPathRenderer,
    ) -> Self {
        AssetManager {
            esbuild_metafile,
            external_assets: Arc::new(Mutex::new(BTreeSet::new())),
            includes: Arc::new(DashSet::new()),
            path_renderer,
            preloads: Arc::new(DashSet::new()),
        }
    }

    pub fn file(&self, asset: &str) -> Result<String, String> {
        let InputProperties { static_paths, .. } = self.input_properties(asset);

        match static_paths.as_slice() {
            [path] => Ok(self.path_renderer.render_path(path)),
            [] => Err(format!("Asset not found: '{asset}'")),
            _ => Err("Unexpectedly multiple assets resolved to the same input".into()),
        }
    }

    pub fn image(&self, asset: &str) -> Result<String, String> {
        let InputProperties { static_paths, .. } = self.input_properties(asset);
        let mut image_paths = static_paths.iter().filter(|path| is_image_path(path));

        if let Some(image_path) = image_paths.next() {
            if image_paths.next().is_some() {
                return Err(format!(
                    "Multiple image assets resolved to the same input: '{asset}'"
                ));
            }

            return Ok(self.path_renderer.render_path(image_path));
        }

        Err(format!("Image asset not found: '{asset}'"))
    }

    fn input_properties(&self, input_path: &str) -> InputProperties {
        match self.esbuild_metafile.input(input_path) {
            InputLookup::Found(input_properties) => input_properties,
            InputLookup::NotFound => InputProperties {
                outputs: Vec::new(),
                static_paths: Vec::new(),
            },
        }
    }

    fn output_preloads(&self, output_path: &str) -> Result<Vec<String>, String> {
        match self.esbuild_metafile.output(output_path) {
            OutputLookup::Found(OutputProperties { preloads }) => Ok(preloads),
            OutputLookup::NotFound => Err(format!("Asset output not found: '{output_path}'")),
        }
    }

    fn register_preloads(&self, preload_paths: Vec<String>) {
        for preload_path in preload_paths {
            self.preloads
                .insert(PreloadableAsset::from_path(preload_path));
        }
    }

    fn rhai_add(&mut self, asset: String) -> Result<(), Box<EvalAltResult>> {
        let InputProperties { outputs, .. } = self.input_properties(&asset);

        if outputs.is_empty() {
            return Err(format!("Asset not found: {asset}").into());
        }

        for output_path in outputs {
            let output_preloads = self.output_preloads(&output_path)?;

            if self.includes.insert(Asset::from_path(output_path)) {
                self.register_preloads(output_preloads);
            }
        }

        Ok(())
    }

    fn rhai_file(&mut self, asset: String) -> Result<String, Box<EvalAltResult>> {
        Ok(self.file(&asset)?)
    }

    fn rhai_image(&mut self, asset: String) -> Result<String, Box<EvalAltResult>> {
        Ok(self.image(&asset)?)
    }

    fn rhai_preload(&mut self, asset: String) -> Result<(), Box<EvalAltResult>> {
        let InputProperties { outputs, .. } = self.input_properties(&asset);

        for output_path in outputs {
            let output_preloads = self.output_preloads(&output_path)?;

            self.preloads
                .insert(PreloadableAsset::from_path(output_path));
            self.register_preloads(output_preloads);
        }

        Ok(())
    }

    fn rhai_render(&mut self) -> String {
        let mut rendered_assets: String = String::new();
        let mut rendered_preloads: BTreeSet<String> = BTreeSet::new();
        let mut rendered_includes: BTreeSet<String> = BTreeSet::new();

        for path in self.preloads.iter() {
            rendered_preloads.insert(path.render(&self.path_renderer));
        }

        for path in self.includes.iter() {
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
            .with_fn("image", Self::rhai_image)
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
            Arc::new(EsbuildMetafile::from_str(metafile_json)?),
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

    const AMBIGUOUS_FAVICON_METAFILE: &str = indoc! {r#"
        {
            "outputs": {
                "static/favicon_ABCDEF12.svg": {
                    "imports": [],
                    "inputs": { "favicon.svg": {} }
                },
                "static/chunk_ABCDEF12.js": {
                    "imports": [],
                    "inputs": { "favicon.svg": {} }
                }
            }
        }
    "#};

    #[test]
    fn image_picks_single_image_among_ambiguous_paths() -> Result<(), anyhow::Error> {
        assert_eq!(
            asset_manager(AMBIGUOUS_FAVICON_METAFILE)?.image("favicon.svg"),
            Ok("/static/favicon_ABCDEF12.svg".to_string())
        );

        Ok(())
    }

    #[test]
    fn image_fails_when_input_resolves_to_multiple_images() -> Result<(), anyhow::Error> {
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
                    }
                }
            }
        "#};

        assert_eq!(
            asset_manager(metafile)?.image("img.png"),
            Err("Multiple image assets resolved to the same input: 'img.png'".to_string())
        );

        Ok(())
    }

    #[test]
    fn image_fails_when_no_image_resolves_to_input() -> Result<(), anyhow::Error> {
        let metafile = indoc! {r#"
            {
                "outputs": {
                    "static/data_AAAAAAAA.js": {
                        "imports": [],
                        "inputs": { "data.js": {} }
                    }
                }
            }
        "#};

        assert_eq!(
            asset_manager(metafile)?.image("data.js"),
            Err("Image asset not found: 'data.js'".to_string())
        );

        Ok(())
    }

    #[test]
    fn image_fails_for_unknown_input() -> Result<(), anyhow::Error> {
        assert_eq!(
            asset_manager(r#"{ "outputs": {} }"#)?.image("missing.png"),
            Err("Image asset not found: 'missing.png'".to_string())
        );

        Ok(())
    }

    #[test]
    fn rhai_image_resolves_image_among_ambiguous_paths() -> Result<(), anyhow::Error> {
        let mut manager = asset_manager(AMBIGUOUS_FAVICON_METAFILE)?;

        assert!(manager.rhai_image("favicon.svg".to_string()).is_ok());

        Ok(())
    }

    #[test]
    fn rhai_image_fails_for_unknown_input() -> Result<(), anyhow::Error> {
        let mut manager = asset_manager(r#"{ "outputs": {} }"#)?;

        assert!(manager.rhai_image("missing.png".to_string()).is_err());

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
    fn file_fails_for_entry_point_only_input() -> Result<(), anyhow::Error> {
        let metafile = indoc! {r#"
            {
                "outputs": {
                    "static/app_ABCDEF12.js": {
                        "imports": [],
                        "entryPoint": "app.ts",
                        "inputs": { "app.ts": {} }
                    }
                }
            }
        "#};

        assert_eq!(
            asset_manager(metafile)?.file("app.ts"),
            Err("Asset not found: 'app.ts'".to_string())
        );

        Ok(())
    }

    #[test]
    fn add_fails_for_static_only_input() -> Result<(), anyhow::Error> {
        let metafile = indoc! {r#"
            {
                "outputs": {
                    "static/logo_ABCDEF12.png": {
                        "imports": [],
                        "inputs": { "logo.png": {} }
                    }
                }
            }
        "#};

        assert!(
            asset_manager(metafile)?
                .rhai_add("logo.png".to_string())
                .is_err()
        );

        Ok(())
    }

    #[test]
    fn preload_registers_known_input_for_rendering() -> Result<(), anyhow::Error> {
        let mut manager = asset_manager(ENTRY_METAFILE)?;

        assert!(manager.rhai_preload("logo.png".to_string()).is_ok());

        assert!(manager.rhai_render().contains("/static/"));

        Ok(())
    }
}
