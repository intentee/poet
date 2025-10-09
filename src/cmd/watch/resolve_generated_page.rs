use std::path::Path as StdPath;
use std::sync::Arc;

use actix_web::Result;
use actix_web::error::ErrorInternalServerError;
use log::error;

use crate::filesystem::Filesystem;
use crate::filesystem::file_entry::FileEntry;
use crate::filesystem::file_entry_stub::FileEntryStub;
use crate::filesystem::read_file_contents_result::ReadFileContentsResult;

pub async fn resolve_generated_page<TFilesystem>(
    filesystem: Arc<TFilesystem>,
    std_path: &StdPath,
    check_for_index: bool,
) -> Result<Option<FileEntry>>
where
    TFilesystem: Filesystem,
{
    match filesystem.read_file_contents(std_path).await {
        Ok(ReadFileContentsResult::Directory) => {
            if check_for_index {
                Box::pin(resolve_generated_page(
                    filesystem.clone(),
                    &std_path.join("index.html"),
                    false,
                ))
                .await
            } else {
                Ok(None)
            }
        }
        Ok(ReadFileContentsResult::Found { contents }) => Ok(Some(
            FileEntryStub {
                contents,
                relative_path: std_path.to_path_buf(),
            }
            .try_into()
            .map_err(ErrorInternalServerError)?,
        )),
        Ok(ReadFileContentsResult::NotFound) => {
            let path_string = std_path.display().to_string();
            let path_str = path_string.as_str();

            if path_str.ends_with('/') && check_for_index {
                return Box::pin(resolve_generated_page(
                    filesystem.clone(),
                    &std_path.join("index.html"),
                    false,
                ))
                .await;
            }

            match path_str {
                "" | "/" if check_for_index => {
                    Box::pin(resolve_generated_page(
                        filesystem.clone(),
                        StdPath::new("index.html"),
                        false,
                    ))
                    .await
                }
                _ => Ok(None),
            }
        }
        Err(err) => {
            let msg = format!("Failed to read file contents: {err}");

            error!("{msg}");

            Err(ErrorInternalServerError(msg))
        }
    }
}
