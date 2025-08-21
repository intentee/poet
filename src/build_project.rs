use std::path::Path;

use anyhow::Result;
use anyhow::anyhow;
use log::info;

use crate::filesystem::Filesystem;
use crate::filesystem::memory::Memory;

pub async fn build_project<TFilesystem>(source_filesystem: &TFilesystem) -> Result<Memory>
where
    TFilesystem: Filesystem,
{
    let files = source_filesystem.read_all_files().await?;

    for file in files {
        info!("Processing file: {file:?}");
    }

    Err(anyhow!("Not implemented yet"))
}
