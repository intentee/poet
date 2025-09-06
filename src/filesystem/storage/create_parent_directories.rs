use std::path::Path;

use anyhow::Result;
use tokio::fs;

pub async fn create_parent_directories(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }

    Ok(())
}
