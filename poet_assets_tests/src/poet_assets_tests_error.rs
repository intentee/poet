use std::io;

use esbuild_metafile::error::Error as EsbuildMetafileError;
use poet_assets::asset_error::AssetError;
use poet_filesystem::filesystem_error::FilesystemError;
use rhai::EvalAltResult;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PoetAssetsTestsError {
    #[error("asset operation failed")]
    Asset(#[from] AssetError),
    #[error("Rhai script failed")]
    EvaluateScript(#[from] Box<EvalAltResult>),
    #[error("filesystem operation failed")]
    Filesystem(#[from] FilesystemError),
    #[error("unable to parse esbuild metafile fixture")]
    ParseMetafileFixture(#[from] EsbuildMetafileError),
    #[error("unable to prepare test fixture on disk")]
    PrepareFixture(#[from] io::Error),
}
