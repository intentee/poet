mod file_entry;
pub mod memory;
pub mod storage;

use std::path::Path;

use anyhow::Result;
use async_trait::async_trait;

use self::file_entry::FileEntry;

#[async_trait]
pub trait Filesystem {
    async fn read_all_files(&self) -> Result<Vec<FileEntry>>;

    async fn set_file_contents(&self, path: &Path, content: &str) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    async fn test_filesystem<TFilesystem>(filesystem: TFilesystem) -> Result<()>
    where
        TFilesystem: Filesystem,
    {
        filesystem
            .set_file_contents(Path::new("test.txt"), "Hello, World! 1")
            .await?;
        filesystem
            .set_file_contents(Path::new("test2.txt"), "Hello, World! 2")
            .await?;
        filesystem
            .set_file_contents(Path::new("test/3.txt"), "Hello, World! 3")
            .await?;
        filesystem
            .set_file_contents(Path::new("test/4/5/6.txt"), "Hello, World! 456")
            .await?;

        let mut files = filesystem.read_all_files().await?;

        files.sort_by(|a, b| a.path.cmp(&b.path));

        assert_eq!(files.len(), 4);

        assert_eq!(
            files[0].path.to_path_buf().display().to_string(),
            "test/3.txt"
        );
        assert_eq!(files[0].contents, "Hello, World! 3");

        assert_eq!(
            files[1].path.to_path_buf().display().to_string(),
            "test/4/5/6.txt"
        );
        assert_eq!(files[1].contents, "Hello, World! 456");

        assert_eq!(
            files[2].path.to_path_buf().display().to_string(),
            "test.txt"
        );
        assert_eq!(files[2].contents, "Hello, World! 1");

        assert_eq!(
            files[3].path.to_path_buf().display().to_string(),
            "test2.txt"
        );
        assert_eq!(files[3].contents, "Hello, World! 2");

        Ok(())
    }

    #[tokio::test]
    async fn test_files_are_set_and_read() -> Result<()> {
        test_filesystem(memory::Memory::default()).await?;

        {
            let base_directory = tempdir()?;

            test_filesystem(storage::Storage {
                base_directory: base_directory.path().to_path_buf(),
            })
            .await?;
        }

        Ok(())
    }
}
