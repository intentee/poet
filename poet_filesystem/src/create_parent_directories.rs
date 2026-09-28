use std::path::Path;

use tokio::fs::create_dir_all;

use crate::filesystem_error::FilesystemError;

pub async fn create_parent_directories(path: &Path) -> Result<(), FilesystemError> {
    match path.parent() {
        Some(parent_directory) => create_dir_all(parent_directory).await.map_err(|source| {
            FilesystemError::CreateParentDirectories {
                path: path.to_path_buf(),
                source,
            }
        }),
        None => Ok(()),
    }
}
