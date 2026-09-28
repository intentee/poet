use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr as _;
use std::sync::Arc;

use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use log::warn;
use poet_filesystem::filesystem::Filesystem;
use poet_filesystem::read_file_contents_result::ReadFileContentsResult;

use crate::asset_error::AssetError;
use crate::esbuild_metafile_path::ESBUILD_METAFILE_PATH;

pub async fn read_esbuild_metafile_or_default<TFilesystem: Filesystem>(
    source_filesystem: &TFilesystem,
) -> Result<Arc<EsbuildMetafile>, AssetError> {
    let esbuild_metafile_path = Path::new(ESBUILD_METAFILE_PATH);

    match source_filesystem
        .read_file_contents(esbuild_metafile_path)
        .await
        .map_err(|source| AssetError::ReadEsbuildMetafile {
            path: PathBuf::from(ESBUILD_METAFILE_PATH),
            source,
        })? {
        ReadFileContentsResult::Directory => Err(AssetError::EsbuildMetafileIsDirectory {
            path: PathBuf::from(ESBUILD_METAFILE_PATH),
        }),
        ReadFileContentsResult::Found { contents } => EsbuildMetafile::from_str(&contents)
            .map(Arc::new)
            .map_err(|source| AssetError::ParseEsbuildMetafile {
                path: PathBuf::from(ESBUILD_METAFILE_PATH),
                source,
            }),
        ReadFileContentsResult::NotFound => {
            warn!("esbuild metafile not found, proceeding without it");

            Ok(Arc::default())
        }
    }
}
