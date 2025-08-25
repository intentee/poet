pub mod file_entry;
pub mod memory;
pub mod read_file_contents_result;
pub mod storage;

use std::path::Path;

use anyhow::Result;
use async_trait::async_trait;

use self::file_entry::FileEntry;
use self::read_file_contents_result::ReadFileContentsResult;

#[async_trait]
pub trait Filesystem {
    async fn read_project_files(&self) -> Result<Vec<FileEntry>>;

    async fn read_file_contents(&self, path: &Path) -> Result<ReadFileContentsResult>;

    async fn set_file_contents(&self, path: &Path, content: &str) -> Result<()>;
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
        println!("{log_prefix} - Creating 'content/test.txt'");
        filesystem
            .set_file_contents(Path::new("content/test.txt"), "Hello, World! 1")
            .await?;

        println!("{log_prefix} - Creating 'content/test2.txt'");
        filesystem
            .set_file_contents(Path::new("content/test2.txt"), "Hello, World! 2")
            .await?;

        println!("{log_prefix} - Creating 'content/test/3.txt'");
        filesystem
            .set_file_contents(Path::new("content/test/3.txt"), "Hello, World! 3")
            .await?;

        println!("{log_prefix} - Creating 'content/test/4/5/6.txt'");
        filesystem
            .set_file_contents(Path::new("content/test/4/5/6.txt"), "Hello, World! 456")
            .await?;

        println!("{log_prefix} - Reading project files");
        let mut files = filesystem.read_project_files().await?;

        println!("{log_prefix} - Sorting project files");
        files.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));

        assert_eq!(files.len(), 4);

        assert_eq!(
            files[0].relative_path.to_path_buf().display().to_string(),
            "content/test/3.txt"
        );
        assert_eq!(files[0].contents, "Hello, World! 3");
        match filesystem
            .read_file_contents(Path::new("content/test/3.txt"))
            .await?
        {
            ReadFileContentsResult::Directory => {
                return Err(anyhow!("Expected file, got directory"));
            }
            ReadFileContentsResult::Found(contents) => assert_eq!(contents, "Hello, World! 3"),
            ReadFileContentsResult::NotFound => return Err(anyhow!("File not found")),
        }

        assert_eq!(
            files[1].relative_path.to_path_buf().display().to_string(),
            "content/test/4/5/6.txt"
        );
        assert_eq!(files[1].contents, "Hello, World! 456");
        match filesystem
            .read_file_contents(Path::new("content/test/4/5/6.txt"))
            .await?
        {
            ReadFileContentsResult::Directory => {
                return Err(anyhow!("Expected file, got directory"));
            }
            ReadFileContentsResult::Found(contents) => assert_eq!(contents, "Hello, World! 456"),
            ReadFileContentsResult::NotFound => return Err(anyhow!("File not found")),
        }

        assert_eq!(
            files[2].relative_path.to_path_buf().display().to_string(),
            "content/test.txt"
        );
        assert_eq!(files[2].contents, "Hello, World! 1");
        match filesystem
            .read_file_contents(Path::new("content/test.txt"))
            .await?
        {
            ReadFileContentsResult::Directory => {
                return Err(anyhow!("Expected file, got directory"));
            }
            ReadFileContentsResult::Found(contents) => assert_eq!(contents, "Hello, World! 1"),
            ReadFileContentsResult::NotFound => return Err(anyhow!("File not found")),
        }

        assert_eq!(
            files[3].relative_path.to_path_buf().display().to_string(),
            "content/test2.txt"
        );
        assert_eq!(files[3].contents, "Hello, World! 2");
        match filesystem
            .read_file_contents(Path::new("content/test2.txt"))
            .await?
        {
            ReadFileContentsResult::Directory => {
                return Err(anyhow!("Expected file, got directory"));
            }
            ReadFileContentsResult::Found(contents) => assert_eq!(contents, "Hello, World! 2"),
            ReadFileContentsResult::NotFound => return Err(anyhow!("File not found")),
        }

        match filesystem
            .read_file_contents(Path::new("test_not_found.txt"))
            .await?
        {
            ReadFileContentsResult::Directory => {
                return Err(anyhow!("Expected file, got directory"));
            }
            ReadFileContentsResult::Found(contents) => {
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
