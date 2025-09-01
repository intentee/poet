use std::collections::BTreeSet;
use std::sync::Arc;

use esbuild_metafile::EsbuildMetaFile;
use esbuild_metafile::HttpPreloader;
use log::warn;
use rhai::CustomType;
use rhai::EvalAltResult;
use rhai::TypeBuilder;

#[derive(Clone)]
pub struct AssetManager {
    esbuild_metafile: Arc<EsbuildMetaFile>,
    http_preloader: Arc<HttpPreloader>,
}

impl AssetManager {
    pub fn add(&mut self, asset: String) {
        if self.http_preloader.register_input(&asset).is_none() {
            warn!("Asset not found: {asset}");
        }
    }

    pub fn from_esbuild_metafile(esbuild_metafile: Arc<EsbuildMetaFile>) -> Self {
        AssetManager {
            esbuild_metafile: esbuild_metafile.clone(),
            http_preloader: Arc::new(HttpPreloader::new(esbuild_metafile)),
        }
    }

    fn rhai_image(&mut self, asset: String) -> Result<String, Box<EvalAltResult>> {
        if let Some(static_paths) = self.esbuild_metafile.find_static_paths_for_input(&asset) {
            if static_paths.len() != 1 {
                return Err("Unexpectedly multiple assets resolved to the same input".into());
            }

            if let Some(path) = static_paths.first() {
                return Ok(format!("/{path}"));
            }
        }

        Err(format!("Asset not found: '{asset}'").into())
    }

    fn rhai_preload(&mut self, asset: String) {
        self.http_preloader.register_preload(&asset);
    }

    fn rhai_render(&mut self) -> String {
        let mut rendered_assets: String = String::new();
        let mut rendered_preloads: BTreeSet<String> = BTreeSet::new();
        let mut rendered_includes: BTreeSet<String> = BTreeSet::new();

        for path in self.http_preloader.preloads.iter() {
            rendered_preloads.insert(path.to_string());
        }

        for path in self.http_preloader.includes.iter() {
            rendered_includes.insert(path.to_string());
        }

        for element in rendered_preloads {
            rendered_assets.push_str(&element);
        }

        for element in rendered_includes {
            rendered_assets.push_str(&element);
        }

        rendered_assets
    }
}

impl CustomType for AssetManager {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("AssetManager")
            .with_fn("add", Self::add)
            .with_fn("image", Self::rhai_image)
            .with_fn("preload", Self::rhai_preload)
            .with_fn("render", Self::rhai_render);
    }
}
