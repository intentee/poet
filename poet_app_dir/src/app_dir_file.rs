use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt as _;
use std::path::PathBuf;

use tokio::fs::set_permissions;
use tokio::fs::write;

use crate::app_dir_error::AppDirError;

pub struct AppDirFile {
    pub contents: String,
    pub path: PathBuf,
    pub permissions_mode: u32,
}

impl AppDirFile {
    pub async fn write(self) -> Result<(), AppDirError> {
        let Self {
            contents,
            path,
            permissions_mode,
        } = self;

        async {
            write(&path, contents).await?;
            set_permissions(&path, Permissions::from_mode(permissions_mode)).await
        }
        .await
        .map_err(|source| AppDirError::WriteAppDirFile { path, source })
    }
}
