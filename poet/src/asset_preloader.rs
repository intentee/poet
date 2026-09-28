use std::sync::Arc;

use dashmap::DashSet;
use esbuild_metafile::asset::Asset;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use esbuild_metafile::input_lookup::InputLookup;
use esbuild_metafile::input_properties::InputProperties;
use esbuild_metafile::output_lookup::OutputLookup;
use esbuild_metafile::output_properties::OutputProperties;
use esbuild_metafile::preloadable_asset::PreloadableAsset;

use crate::asset_registration_result::AssetRegistrationResult;

pub struct AssetPreloader {
    esbuild_metafile: Arc<EsbuildMetafile>,
    pub includes: DashSet<Asset>,
    pub preloads: DashSet<PreloadableAsset>,
}

impl AssetPreloader {
    pub fn new(esbuild_metafile: Arc<EsbuildMetafile>) -> Self {
        Self {
            esbuild_metafile,
            includes: DashSet::new(),
            preloads: DashSet::new(),
        }
    }

    pub fn register_input(&self, input_path: &str) -> AssetRegistrationResult {
        let output_paths = self.find_output_paths_for_input(input_path);

        if output_paths.is_empty() {
            return AssetRegistrationResult::NotFound;
        }

        for output_path in output_paths {
            if self.includes.insert(Asset::from_path(output_path.clone())) {
                self.register_preloads_of_output(&output_path);
            }
        }

        AssetRegistrationResult::Registered
    }

    pub fn register_preload(&self, input_path: &str) {
        for output_path in self.find_output_paths_for_input(input_path) {
            self.preloads
                .insert(PreloadableAsset::from_path(output_path.clone()));
            self.register_preloads_of_output(&output_path);
        }
    }

    fn find_output_paths_for_input(&self, input_path: &str) -> Vec<String> {
        match self.esbuild_metafile.input(input_path) {
            InputLookup::Found(InputProperties { outputs, .. }) => outputs,
            InputLookup::NotFound => Vec::new(),
        }
    }

    fn register_preloads_of_output(&self, output_path: &str) {
        if let OutputLookup::Found(OutputProperties { preloads }) =
            self.esbuild_metafile.output(output_path)
        {
            for preload_path in preloads {
                self.preloads
                    .insert(PreloadableAsset::from_path(preload_path));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use anyhow::Result;
    use indoc::indoc;

    use super::*;

    fn asset_preloader(metafile_json: &str) -> Result<AssetPreloader> {
        Ok(AssetPreloader::new(Arc::new(EsbuildMetafile::from_str(
            metafile_json,
        )?)))
    }

    #[test]
    fn register_input_includes_outputs_and_preloads_their_imports() -> Result<()> {
        let preloader = asset_preloader(indoc! {r#"
            {
                "outputs": {
                    "dist/main.js": {
                        "imports": [
                            { "path": "dist/chunk1.js" },
                            { "path": "dist/chunk2.js" }
                        ],
                        "cssBundle": "dist/main.css",
                        "entryPoint": "src/main.ts",
                        "inputs": {}
                    },
                    "dist/main.css": {
                        "imports": [
                            { "path": "dist/style1.css" },
                            { "path": "dist/style1.css" }
                        ],
                        "entryPoint": "src/style.css",
                        "inputs": {}
                    }
                }
            }
        "#})?;

        assert_eq!(
            preloader.register_input("src/main.ts"),
            AssetRegistrationResult::Registered
        );

        assert_eq!(preloader.includes.len(), 2);
        assert!(
            preloader
                .includes
                .contains(&Asset::from_path("dist/main.js".to_string()))
        );
        assert!(
            preloader
                .includes
                .contains(&Asset::from_path("dist/main.css".to_string()))
        );

        assert_eq!(preloader.preloads.len(), 3);
        assert!(
            preloader
                .preloads
                .contains(&PreloadableAsset::from_path("dist/chunk1.js".to_string()))
        );
        assert!(
            preloader
                .preloads
                .contains(&PreloadableAsset::from_path("dist/chunk2.js".to_string()))
        );
        assert!(
            preloader
                .preloads
                .contains(&PreloadableAsset::from_path("dist/style1.css".to_string()))
        );

        Ok(())
    }

    #[test]
    fn register_input_reports_not_found_for_input_with_only_static_paths() -> Result<()> {
        let preloader = asset_preloader(indoc! {r#"
            {
                "outputs": {
                    "static/logo_ABCDEF12.png": {
                        "imports": [],
                        "inputs": { "logo.png": {} }
                    }
                }
            }
        "#})?;

        assert_eq!(
            preloader.register_input("logo.png"),
            AssetRegistrationResult::NotFound
        );
        assert!(preloader.includes.is_empty());

        Ok(())
    }
}
