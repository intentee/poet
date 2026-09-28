use std::collections::BTreeSet;
use std::sync::Arc;

use dashmap::DashSet;
use esbuild_metafile::asset::Asset;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use esbuild_metafile::output_lookup::OutputLookup;
use esbuild_metafile::output_properties::OutputProperties;
use esbuild_metafile::preloadable_asset::PreloadableAsset;

use crate::asset_error::AssetError;
use crate::asset_input::AssetInput;
use crate::asset_path_renderer::AssetPathRenderer;

#[derive(Clone)]
pub struct AssetPreloader {
    esbuild_metafile: Arc<EsbuildMetafile>,
    includes: Arc<DashSet<Asset>>,
    preloads: Arc<DashSet<PreloadableAsset>>,
}

impl AssetPreloader {
    #[must_use]
    pub fn new(esbuild_metafile: Arc<EsbuildMetafile>) -> Self {
        Self {
            esbuild_metafile,
            includes: Arc::default(),
            preloads: Arc::default(),
        }
    }

    pub fn include(
        &self,
        AssetInput {
            input_path,
            outputs,
            ..
        }: &AssetInput,
    ) -> Result<(), AssetError> {
        if outputs.is_empty() {
            return Err(AssetError::AssetHasNoOutput {
                input_path: input_path.clone(),
            });
        }

        for output_path in outputs {
            self.register_output_preloads(output_path)?;
            self.includes.insert(Asset::from_path(output_path.clone()));
        }

        Ok(())
    }

    pub fn preload(
        &self,
        AssetInput {
            input_path,
            outputs,
            ..
        }: &AssetInput,
    ) -> Result<(), AssetError> {
        if outputs.is_empty() {
            return Err(AssetError::AssetHasNoOutput {
                input_path: input_path.clone(),
            });
        }

        for output_path in outputs {
            self.register_output_preloads(output_path)?;
            self.preloads
                .insert(PreloadableAsset::from_path(output_path.clone()));
        }

        Ok(())
    }

    #[must_use]
    pub fn render_includes(&self, path_renderer: &AssetPathRenderer) -> BTreeSet<String> {
        self.includes
            .iter()
            .map(|include| include.render(path_renderer))
            .collect()
    }

    #[must_use]
    pub fn render_preloads(&self, path_renderer: &AssetPathRenderer) -> BTreeSet<String> {
        self.preloads
            .iter()
            .map(|preload| preload.render(path_renderer))
            .collect()
    }

    fn register_output_preloads(&self, output_path: &str) -> Result<(), AssetError> {
        match self.esbuild_metafile.output(output_path) {
            OutputLookup::Found(OutputProperties { preloads }) => {
                for preload_path in preloads {
                    self.preloads
                        .insert(PreloadableAsset::from_path(preload_path));
                }

                Ok(())
            }
            OutputLookup::NotFound => Err(AssetError::AssetOutputNotFound {
                output_path: output_path.to_owned(),
            }),
        }
    }
}
