use std::sync::Arc;

use esbuild_metafile::EsbuildMetaFile;
use esbuild_metafile::HttpPreloader;
use rhai::CustomType;
use rhai::TypeBuilder;

#[derive(Clone)]
pub struct AssetManager {
    http_preloader: Arc<HttpPreloader>,
}

impl AssetManager {
    pub fn add(&mut self, asset: String) {
        self.http_preloader.register_input(&asset);
    }

    pub fn from_esbuild_metafile(esbuild_metafile: Arc<EsbuildMetaFile>) -> Self {
        AssetManager {
            http_preloader: Arc::new(HttpPreloader::new(esbuild_metafile)),
        }
    }

    pub fn preload(&mut self, asset: String) {
        self.http_preloader.register_preload(&asset);
    }

    pub fn render(&mut self) -> String {
        let mut rendered_assets: String = String::new();

        for path in self.http_preloader.preloads.iter() {
            rendered_assets.push_str(&path.to_string());
        }

        for path in self.http_preloader.includes.iter() {
            rendered_assets.push_str(&path.to_string());
        }

        rendered_assets
    }
}

impl CustomType for AssetManager {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("AssetManager")
            .with_fn("add", Self::add)
            .with_fn("preload", Self::preload)
            .with_fn("render", Self::render);
    }
}
