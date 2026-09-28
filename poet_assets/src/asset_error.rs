use std::io;
use std::path::PathBuf;

use esbuild_metafile::error::Error as EsbuildMetafileError;
use poet_filesystem::filesystem_error::FilesystemError;
use rhai::EvalAltResult;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AssetError {
    #[error("asset '{input_path}' has multiple image files")]
    AssetHasMultipleImages { input_path: String },
    #[error("asset '{input_path}' has multiple static files")]
    AssetHasMultipleStaticFiles { input_path: String },
    #[error("asset '{input_path}' has no image file")]
    AssetHasNoImage { input_path: String },
    #[error("asset '{input_path}' has no bundled output to include")]
    AssetHasNoOutput { input_path: String },
    #[error("asset '{input_path}' has no static file")]
    AssetHasNoStaticFile { input_path: String },
    #[error("asset '{input_path}' is not in the esbuild metafile")]
    AssetNotFound { input_path: String },
    #[error("asset output '{output_path}' is not in the esbuild metafile")]
    AssetOutputNotFound { output_path: String },
    #[error("unable to copy asset '{source_path}' to '{target_path}'")]
    CopyAsset {
        source_path: PathBuf,
        target_path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("unable to create a directory for asset '{target_path}'")]
    CreateAssetDirectory {
        target_path: PathBuf,
        #[source]
        source: FilesystemError,
    },
    #[error("esbuild metafile '{path}' is a directory, not a file")]
    EsbuildMetafileIsDirectory { path: PathBuf },
    #[error("unable to parse esbuild metafile '{path}'")]
    ParseEsbuildMetafile {
        path: PathBuf,
        #[source]
        source: EsbuildMetafileError,
    },
    #[error("unable to read esbuild metafile '{path}'")]
    ReadEsbuildMetafile {
        path: PathBuf,
        #[source]
        source: FilesystemError,
    },
}

impl From<AssetError> for Box<EvalAltResult> {
    fn from(asset_error: AssetError) -> Self {
        Self::new(EvalAltResult::ErrorSystem(
            String::new(),
            Box::new(asset_error),
        ))
    }
}
