pub mod create_parent_directories;

use std::path::Path;
use std::path::PathBuf;

use anyhow::Context as _;
use anyhow::Result;
use anyhow::anyhow;
use async_trait::async_trait;
use log::debug;
use tokio::fs;

use super::Filesystem;
use super::file_entry::FileEntry;
use super::read_file_contents_result::ReadFileContentsResult;
use crate::blog_name::BlogName;
use crate::filesystem::file_entry_stub::FileEntryStub;
use crate::filesystem::storage::create_parent_directories::create_parent_directories;

struct ReadFilesFromDirResult {
    pub directories: Vec<PathBuf>,
    pub files: Vec<FileEntry>,
}

pub struct Storage {
    pub base_directory: PathBuf,
}

impl Storage {
    async fn read_files_from_dir(&self, dir: PathBuf) -> Result<ReadFilesFromDirResult> {
        let mut entries = fs::read_dir(dir).await?;

        let mut directories = Vec::new();
        let mut files = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let metadata = entry.metadata().await?;

            if metadata.is_dir() {
                directories.push(path);
            } else {
                let relative_path = path
                    .strip_prefix(&self.base_directory)
                    .map_err(|_| {
                        anyhow!(
                            "Unable to strip base directory prefix from '{}', base directory: {}",
                            path.display(),
                            self.base_directory.display()
                        )
                    })?
                    .to_path_buf();

                if let Some(extension) = path.extension() {
                    match extension.to_str() {
                        Some("md") | Some("rhai") | Some("toml") => {
                            files.push(
                                FileEntryStub {
                                    contents: fs::read_to_string(&path).await.context(format!(
                                        "Failed to read file: {}",
                                        path.display()
                                    ))?,
                                    relative_path,
                                }
                                .try_into()?,
                            );
                        }
                        Some(_) => debug!("Skipping path: {}", path.display()),
                        None => {}
                    }
                }
            }
        }

        Ok(ReadFilesFromDirResult { directories, files })
    }

    async fn read_files_from_dirs(&self, to_visit: Vec<PathBuf>) -> Result<Vec<FileEntry>> {
        let mut all_files = Vec::new();
        let mut to_visit_mut = to_visit.clone();

        while let Some(current) = to_visit_mut.pop() {
            if !current.exists() {
                continue;
            }

            let ReadFilesFromDirResult { directories, files } =
                self.read_files_from_dir(current).await?;

            all_files.extend(files);
            to_visit_mut.extend(directories);
        }

        Ok(all_files)
    }
}

#[async_trait]
impl Filesystem for Storage {
    async fn read_author_files(&self) -> Result<Vec<FileEntry>> {
        let to_visit: Vec<PathBuf> = vec![self.base_directory.join("authors")];

        Ok(self
            .read_files_from_dirs(to_visit)
            .await?
            .into_iter()
            .filter(|file_entry| file_entry.kind.is_author())
            .collect::<Vec<FileEntry>>())
    }

    async fn read_blog_config_files(&self) -> Result<Vec<FileEntry>> {
        let ReadFilesFromDirResult { files, .. } = self
            .read_files_from_dir(self.base_directory.join("blogs"))
            .await?;

        Ok(files
            .into_iter()
            .filter(|file_entry| file_entry.kind.is_blog_config())
            .collect::<Vec<FileEntry>>())
    }

    async fn read_blog_posts_from_blog(&self, blog_name: &BlogName) -> Result<Vec<FileEntry>> {
        let ReadFilesFromDirResult { files, .. } = self
            .read_files_from_dir(
                self.base_directory
                    .join(blog_name.relative_blog_directory()),
            )
            .await?;

        Ok(files
            .into_iter()
            .filter(|file_entry| file_entry.kind.is_blog_post())
            .collect::<Vec<FileEntry>>())
    }

    async fn read_content_files(&self) -> Result<Vec<FileEntry>> {
        let to_visit: Vec<PathBuf> = vec![self.base_directory.join("content")];

        Ok(self
            .read_files_from_dirs(to_visit)
            .await?
            .into_iter()
            .filter(|file_entry| file_entry.kind.is_content())
            .collect::<Vec<FileEntry>>())
    }

    async fn read_project_files(&self) -> Result<Vec<FileEntry>> {
        let to_visit: Vec<PathBuf> = vec![
            self.base_directory.join("authors"),
            self.base_directory.join("blogs"),
            self.base_directory.join("content"),
            self.base_directory.join("prompts"),
            self.base_directory.join("shortcodes"),
        ];

        self.read_files_from_dirs(to_visit).await
    }

    async fn read_prompt_files(&self) -> Result<Vec<FileEntry>> {
        let to_visit: Vec<PathBuf> = vec![self.base_directory.join("prompts")];

        Ok(self
            .read_files_from_dirs(to_visit)
            .await?
            .into_iter()
            .filter(|file_entry| file_entry.kind.is_prompt())
            .collect::<Vec<FileEntry>>())
    }

    async fn read_shortcode_files(&self) -> Result<Vec<FileEntry>> {
        let to_visit: Vec<PathBuf> = vec![self.base_directory.join("shortcodes")];

        Ok(self
            .read_files_from_dirs(to_visit)
            .await?
            .into_iter()
            .filter(|file_entry| file_entry.kind.is_shortcode())
            .collect::<Vec<FileEntry>>())
    }

    async fn read_file_contents(&self, relative_path: &Path) -> Result<ReadFileContentsResult> {
        let full_path = self.base_directory.join(relative_path);

        if !full_path.exists() {
            return Ok(ReadFileContentsResult::NotFound);
        }

        if full_path.is_dir() {
            return Ok(ReadFileContentsResult::Directory);
        }

        let contents = fs::read_to_string(&full_path).await?;

        Ok(ReadFileContentsResult::Found { contents })
    }

    async fn set_file_contents(&self, path: &Path, contents: &str) -> Result<()> {
        let full_path = self.base_directory.join(path);

        create_parent_directories(&full_path).await?;

        fs::write(&full_path, contents)
            .await
            .context(format!("Failed to write file: {}", full_path.display()))?;

        Ok(())
    }

    fn set_file_contents_sync(&self, _: &Path, _: &str) -> Result<()> {
        unreachable!("This should not be used with storage filesystem")
    }
}
