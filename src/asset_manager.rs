use std::path::PathBuf;
use std::sync::Arc;

use esbuild_metafile::EsbuildMetaFile;
use esbuild_metafile::HttpPreloader;
use rhai::CustomType;
use rhai::TypeBuilder;

#[derive(Clone)]
pub struct AssetManager {
    http_preloader: Arc<HttpPreloader>,
    is_watching: bool,
    target_file_relative_path: PathBuf,
}

impl AssetManager {
    pub fn add(&mut self, asset: String) {
        self.http_preloader.register_input(&asset);
    }

    pub fn from_esbuild_metafile(
        esbuild_metafile: Arc<EsbuildMetaFile>,
        is_watching: bool,
        target_file_relative_path: PathBuf,
    ) -> Self {
        AssetManager {
            http_preloader: Arc::new(HttpPreloader::new(esbuild_metafile)),
            is_watching,
            target_file_relative_path,
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

        if self.is_watching {
            let relative_path = self.target_file_relative_path.to_string_lossy();

            rendered_assets.push_str(&format!(r#"<script async id="poet-live-reload" data-relative-path="{relative_path}" src="/api/v1/live_reload_script.js" type="module"></script>"#));
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
