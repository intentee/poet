pub mod file_entry;
pub mod memory;
pub mod read_file_contents_result;
pub mod storage;

use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use self::file_entry::FileEntry;
use self::read_file_contents_result::ReadFileContentsResult;

#[async_trait]
pub trait Filesystem: Send + Sync {
    async fn read_content_files(&self) -> Result<Vec<FileEntry>>;

    async fn read_file_contents(&self, path: &Path) -> Result<ReadFileContentsResult>;

    async fn set_file_contents(&self, path: &Path, contents: &str) -> Result<()>;

    fn set_file_contents_sync(&self, path: &Path, contents: &str) -> Result<()>;

    async fn copy_from<TFilesystem: Filesystem>(&self, other: Arc<TFilesystem>) -> Result<()> {
        for FileEntry {
            contents,
            relative_path,
        } in other.read_content_files().await?
        {
            self.set_file_contents(&relative_path, &contents).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use tempfile::tempdir;

    use super::*;

    async fn test_filesystem<TFilesystem>(log_prefix: &str, filesystem: TFilesystem) -> Result<()>
    where
        TFilesystem: Filesystem,
    {
        println!("{log_prefix} - Creating 'content/test.md'");
        filesystem
            .set_file_contents(Path::new("content/test.md"), "Hello, World! 1")
            .await?;

        println!("{log_prefix} - Creating 'content/test2.md'");
        filesystem
            .set_file_contents(Path::new("content/test2.md"), "Hello, World! 2")
            .await?;

        println!("{log_prefix} - Creating 'content/test/3.md'");
        filesystem
            .set_file_contents(Path::new("content/test/3.md"), "Hello, World! 3")
            .await?;

        println!("{log_prefix} - Creating 'content/test/4/5/6.md'");
        filesystem
            .set_file_contents(Path::new("content/test/4/5/6.md"), "Hello, World! 456")
            .await?;

        println!("{log_prefix} - Reading project files");
        let mut files = filesystem.read_content_files().await?;

        println!("{log_prefix} - Sorting project files");
        files.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));

        assert_eq!(files.len(), 4);

        assert_eq!(
            files[0].relative_path.to_path_buf().display().to_string(),
            "content/test/3.md"
        );
        assert_eq!(files[0].contents, "Hello, World! 3");
        match filesystem
            .read_file_contents(Path::new("content/test/3.md"))
            .await?
        {
            ReadFileContentsResult::Directory => {
                return Err(anyhow!("Expected file, got directory"));
            }
            ReadFileContentsResult::Found { contents } => assert_eq!(contents, "Hello, World! 3"),
            ReadFileContentsResult::NotFound => return Err(anyhow!("File not found")),
        }

        assert_eq!(
            files[1].relative_path.to_path_buf().display().to_string(),
            "content/test/4/5/6.md"
        );
        assert_eq!(files[1].contents, "Hello, World! 456");
        match filesystem
            .read_file_contents(Path::new("content/test/4/5/6.md"))
            .await?
        {
            ReadFileContentsResult::Directory => {
                return Err(anyhow!("Expected file, got directory"));
            }
            ReadFileContentsResult::Found { contents } => assert_eq!(contents, "Hello, World! 456"),
            ReadFileContentsResult::NotFound => return Err(anyhow!("File not found")),
        }

        assert_eq!(
            files[2].relative_path.to_path_buf().display().to_string(),
            "content/test.md"
        );
        assert_eq!(files[2].contents, "Hello, World! 1");
        match filesystem
            .read_file_contents(Path::new("content/test.md"))
            .await?
        {
            ReadFileContentsResult::Directory => {
                return Err(anyhow!("Expected file, got directory"));
            }
            ReadFileContentsResult::Found { contents } => assert_eq!(contents, "Hello, World! 1"),
            ReadFileContentsResult::NotFound => return Err(anyhow!("File not found")),
        }

        assert_eq!(
            files[3].relative_path.to_path_buf().display().to_string(),
            "content/test2.md"
        );
        assert_eq!(files[3].contents, "Hello, World! 2");
        match filesystem
            .read_file_contents(Path::new("content/test2.md"))
            .await?
        {
            ReadFileContentsResult::Directory => {
                return Err(anyhow!("Expected file, got directory"));
            }
            ReadFileContentsResult::Found { contents } => assert_eq!(contents, "Hello, World! 2"),
            ReadFileContentsResult::NotFound => return Err(anyhow!("File not found")),
        }

        match filesystem
            .read_file_contents(Path::new("test_not_found.md"))
            .await?
        {
            ReadFileContentsResult::Directory => {
                return Err(anyhow!("Expected file, got directory"));
            }
            ReadFileContentsResult::Found { contents } => {
                return Err(anyhow!("File should not be found: {contents}"));
            }
            ReadFileContentsResult::NotFound => {}
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_files_are_set_and_read() -> Result<()> {
        test_filesystem("Memory", memory::Memory::default()).await?;

        {
            let base_directory = tempdir()?;

            test_filesystem(
                "Storage",
                storage::Storage {
                    base_directory: base_directory.path().to_path_buf(),
                },
            )
            .await?;
        }

        Ok(())
    }
}
