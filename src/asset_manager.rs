use std::collections::BTreeSet;
use std::sync::Arc;

use esbuild_metafile::EsbuildMetaFile;
use esbuild_metafile::HttpPreloader;
use esbuild_metafile::renders_path::RendersPath;
use log::warn;
use rhai::CustomType;
use rhai::EvalAltResult;
use rhai::TypeBuilder;

use crate::asset_path_renderer::AssetPathRenderer;

#[derive(Clone)]
pub struct AssetManager {
    esbuild_metafile: Arc<EsbuildMetaFile>,
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
            http_preloader: Arc::new(HttpPreloader::new(esbuild_metafile)),
            path_renderer,
        }
    }

    pub fn add(&mut self, asset: String) {
        if self.http_preloader.register_input(&asset).is_none() {
            warn!("Asset not found: {asset}");
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
            .with_fn("file", Self::rhai_file)
            .with_fn("preload", Self::rhai_preload)
            .with_fn("render", Self::rhai_render);
    }
}
