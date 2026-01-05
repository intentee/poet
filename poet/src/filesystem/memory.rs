use std::path::Path;

use anyhow::Result;
use anyhow::anyhow;
use async_trait::async_trait;
use dashmap::DashMap;

use super::Filesystem;
use super::file_entry::FileEntry;
use super::read_file_contents_result::ReadFileContentsResult;
use crate::blog_name::BlogName;
use crate::filesystem::file_entry_stub::FileEntryStub;

pub struct Memory {
    files: DashMap<String, String>,
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            files: DashMap::new(),
        }
    }
}

#[async_trait]
impl Filesystem for Memory {
    async fn read_author_files(&self) -> Result<Vec<FileEntry>> {
        Ok(self
            .read_project_files()
            .await?
            .into_iter()
            .filter(|file_entry| file_entry.kind.is_author())
            .collect::<Vec<FileEntry>>())
    }

    async fn read_blog_config_files(&self) -> Result<Vec<FileEntry>> {
        Ok(self
            .read_project_files()
            .await?
            .into_iter()
            .filter(|file_entry| file_entry.kind.is_blog_config())
            .collect::<Vec<FileEntry>>())
    }

    async fn read_blog_posts_from_blog(&self, blog_name: &BlogName) -> Result<Vec<FileEntry>> {
        let blog_dir = blog_name.relative_blog_directory();

        Ok(self
            .read_project_files()
            .await?
            .into_iter()
            .filter(|file_entry| {
                file_entry.kind.is_blog_post() && file_entry.relative_path.starts_with(&blog_dir)
            })
            .collect::<Vec<FileEntry>>())
    }

    async fn read_content_files(&self) -> Result<Vec<FileEntry>> {
        Ok(self
            .read_project_files()
            .await?
            .into_iter()
            .filter(|file_entry| file_entry.kind.is_content())
            .collect::<Vec<FileEntry>>())
    }

    async fn read_project_files(&self) -> Result<Vec<FileEntry>> {
        self.files
            .iter()
            .map(|entry| {
                FileEntryStub {
                    contents: entry.value().clone(),
                    relative_path: entry.key().into(),
                }
                .try_into()
            })
            .collect::<Result<Vec<FileEntry>>>()
    }

    async fn read_prompt_files(&self) -> Result<Vec<FileEntry>> {
        Ok(self
            .read_project_files()
            .await?
            .into_iter()
            .filter(|file_entry| file_entry.kind.is_prompt())
            .collect::<Vec<FileEntry>>())
    }

    async fn read_shortcode_files(&self) -> Result<Vec<FileEntry>> {
        Ok(self
            .read_project_files()
            .await?
            .into_iter()
            .filter(|file_entry| file_entry.kind.is_shortcode())
            .collect::<Vec<FileEntry>>())
    }

    async fn read_file_contents(&self, relative_path: &Path) -> Result<ReadFileContentsResult> {
        let path_str = relative_path
            .to_str()
            .ok_or_else(|| anyhow!("Unable to stringify path"))?;

        if let Some(contents) = self.files.get(path_str) {
            Ok(ReadFileContentsResult::Found {
                contents: contents.value().to_owned(),
            })
        } else {
            Ok(ReadFileContentsResult::NotFound)
        }
    }

    async fn set_file_contents(&self, path: &Path, contents: &str) -> Result<()> {
        self.set_file_contents_sync(path, contents)
    }

    fn set_file_contents_sync(&self, path: &Path, contents: &str) -> Result<()> {
        self.files.insert(
            path.to_str()
                .ok_or_else(|| anyhow!("Unable to stringify path"))?
                .to_string(),
            contents.to_string(),
        );

        Ok(())
    }
}
