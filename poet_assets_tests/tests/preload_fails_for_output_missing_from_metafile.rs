use std::sync::Arc;

use poet_assets::asset_error::AssetError;
use poet_assets::asset_input::AssetInput;
use poet_assets::asset_preloader::AssetPreloader;

#[test]
fn preload_fails_for_output_missing_from_metafile() {
    assert!(matches!(
        AssetPreloader::new(Arc::default()).preload(&AssetInput {
            input_path: "app.ts".to_owned(),
            outputs: vec!["static/missing_ABCDEF12.js".to_owned()],
            static_paths: vec![],
        }),
        Err(AssetError::AssetOutputNotFound { output_path }) if output_path == "static/missing_ABCDEF12.js"
    ));
}
