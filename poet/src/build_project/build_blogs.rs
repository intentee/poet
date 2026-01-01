use std::sync::Arc;

use anyhow::Result;
use log::info;

use crate::blog_config::BlogConfig;
use crate::filesystem::Filesystem as _;
use crate::filesystem::storage::Storage;

pub async fn build_blogs(source_filesystem: Arc<Storage>) -> Result<()> {
    for file in source_filesystem.read_blog_config_files().await? {
        let config: BlogConfig = toml::from_str(&file.contents)?;

        info!("Found blog config: {:?}", config);
    }

    Ok(())
}
