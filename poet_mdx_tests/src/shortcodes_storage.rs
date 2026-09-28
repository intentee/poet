use std::path::Path;

use poet_filesystem::filesystem::Filesystem as _;
use poet_filesystem::storage::Storage;
use tempfile::TempDir;
use tempfile::tempdir;

use crate::poet_mdx_tests_error::PoetMdxTestsError;

pub struct ShortcodesStorage {
    pub directory: TempDir,
    pub storage: Storage,
}

impl ShortcodesStorage {
    pub async fn with_shortcode(
        shortcode_name: &str,
        shortcode_source: &str,
    ) -> Result<Self, PoetMdxTestsError> {
        let directory = tempdir()?;
        let storage = Storage {
            base_directory: directory.path().to_path_buf(),
        };

        storage
            .set_file_contents(
                &Path::new("shortcodes").join(format!("{shortcode_name}.rhai")),
                shortcode_source,
            )
            .await?;

        Ok(Self { directory, storage })
    }
}
