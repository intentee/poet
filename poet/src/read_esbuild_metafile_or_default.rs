use std::path::PathBuf;
use std::str::FromStr as _;
use std::sync::Arc;

use anyhow::Result;
use anyhow::anyhow;
use esbuild_metafile::EsbuildMetaFile;
use log::warn;

use crate::filesystem::Filesystem as _;
use crate::filesystem::read_file_contents_result::ReadFileContentsResult;
use crate::filesystem::storage::Storage;

pub async fn read_esbuild_metafile_or_default(
    source_filesystem: Arc<Storage>,
) -> Result<Arc<EsbuildMetaFile>> {
    Ok(match source_filesystem
        .read_file_contents(&PathBuf::from("esbuild-meta.json"))
        .await?
    {
        ReadFileContentsResult::Directory => {
            return Err(anyhow!(
                "esbuild metafile should be a file, not a directory"
            ));
        }
        ReadFileContentsResult::Found { contents } => EsbuildMetaFile::from_str(&contents)?,
        ReadFileContentsResult::NotFound => {
            warn!("esbuild metafile not found, proceeding without it");

            EsbuildMetaFile::default()
        }
    }
    .into())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use tempfile::tempdir;

    use super::*;
    use crate::asset_manager::AssetManager;
    use crate::asset_path_renderer::AssetPathRenderer;

    const METAFILE: &str = indoc! {r#"
        {
            "outputs": {
                "static/logo_ABCDEF12.png": {
                    "imports": [],
                    "inputs": { "logo.png": {} }
                }
            }
        }
    "#};

    fn asset_manager(metafile: Arc<EsbuildMetaFile>) -> AssetManager {
        AssetManager::from_esbuild_metafile(
            metafile,
            AssetPathRenderer {
                base_path: "/".to_string(),
            },
        )
    }

    #[tokio::test]
    async fn reads_and_parses_an_existing_metafile() -> Result<()> {
        let directory = tempdir()?;
        let storage = Storage {
            base_directory: directory.path().to_path_buf(),
        };

        storage
            .set_file_contents(&PathBuf::from("esbuild-meta.json"), METAFILE)
            .await?;

        let metafile = read_esbuild_metafile_or_default(Arc::new(storage)).await?;

        assert_eq!(
            asset_manager(metafile).file("logo.png"),
            Ok("/static/logo_ABCDEF12.png".to_string())
        );

        Ok(())
    }

    #[tokio::test]
    async fn falls_back_to_default_when_metafile_is_missing() -> Result<()> {
        let directory = tempdir()?;
        let storage = Storage {
            base_directory: directory.path().to_path_buf(),
        };

        let metafile = read_esbuild_metafile_or_default(Arc::new(storage)).await?;

        assert!(asset_manager(metafile).file("logo.png").is_err());

        Ok(())
    }

    #[tokio::test]
    async fn errors_when_metafile_path_is_a_directory() -> Result<()> {
        let directory = tempdir()?;
        let storage = Storage {
            base_directory: directory.path().to_path_buf(),
        };

        std::fs::create_dir(directory.path().join("esbuild-meta.json"))?;

        assert!(
            read_esbuild_metafile_or_default(Arc::new(storage))
                .await
                .is_err()
        );

        Ok(())
    }
}
