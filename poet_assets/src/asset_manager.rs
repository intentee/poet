use std::sync::Arc;

use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use rhai::CustomType;
use rhai::EvalAltResult;
use rhai::TypeBuilder;

use crate::asset_error::AssetError;
use crate::asset_input_resolver::AssetInputResolver;
use crate::asset_path_renderer::AssetPathRenderer;
use crate::asset_preloader::AssetPreloader;
use crate::external_asset::ExternalAsset;
use crate::external_asset_collection::ExternalAssetCollection;
use crate::static_asset_resolver::StaticAssetResolver;

#[derive(Clone)]
pub struct AssetManager {
    pub asset_input_resolver: AssetInputResolver,
    pub asset_preloader: AssetPreloader,
    pub external_asset_collection: ExternalAssetCollection,
    pub static_asset_resolver: StaticAssetResolver,
}

impl AssetManager {
    #[must_use]
    pub fn from_esbuild_metafile(
        esbuild_metafile: Arc<EsbuildMetafile>,
        path_renderer: AssetPathRenderer,
    ) -> Self {
        Self {
            asset_input_resolver: AssetInputResolver {
                esbuild_metafile: esbuild_metafile.clone(),
            },
            asset_preloader: AssetPreloader::new(esbuild_metafile),
            external_asset_collection: ExternalAssetCollection::default(),
            static_asset_resolver: StaticAssetResolver { path_renderer },
        }
    }

    pub fn file(&self, input_path: &str) -> Result<String, AssetError> {
        self.static_asset_resolver
            .file(&self.asset_input_resolver.resolve(input_path)?)
    }

    pub fn image(&self, input_path: &str) -> Result<String, AssetError> {
        self.static_asset_resolver
            .image(&self.asset_input_resolver.resolve(input_path)?)
    }

    #[must_use]
    pub fn render(&self) -> String {
        let path_renderer = &self.static_asset_resolver.path_renderer;
        let mut rendered_includes = self.asset_preloader.render_includes(path_renderer);

        rendered_includes.extend(self.external_asset_collection.render(path_renderer));

        self.asset_preloader
            .render_preloads(path_renderer)
            .into_iter()
            .chain(rendered_includes)
            .collect()
    }

    fn rhai_add(&mut self, input_path: &str) -> Result<(), Box<EvalAltResult>> {
        Ok(self
            .asset_preloader
            .include(&self.asset_input_resolver.resolve(input_path)?)?)
    }

    fn rhai_file(&mut self, input_path: &str) -> Result<String, Box<EvalAltResult>> {
        Ok(self.file(input_path)?)
    }

    fn rhai_image(&mut self, input_path: &str) -> Result<String, Box<EvalAltResult>> {
        Ok(self.image(input_path)?)
    }

    fn rhai_preload(&mut self, input_path: &str) -> Result<(), Box<EvalAltResult>> {
        Ok(self
            .asset_preloader
            .preload(&self.asset_input_resolver.resolve(input_path)?)?)
    }

    fn rhai_render(&mut self) -> String {
        self.render()
    }

    fn rhai_script(&mut self, url: String) {
        self.external_asset_collection
            .add(ExternalAsset::Script(url));
    }

    fn rhai_stylesheet(&mut self, url: String) {
        self.external_asset_collection
            .add(ExternalAsset::Stylesheet(url));
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
