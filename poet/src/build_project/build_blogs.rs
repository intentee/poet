use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use anyhow::anyhow;
use log::info;

use crate::blog_config::BlogConfig;
use crate::blog_name::BlogName;
use crate::document_error_collection::DocumentErrorCollection;
use crate::filesystem::Filesystem as _;
use crate::filesystem::storage::Storage;

pub async fn build_blogs(source_filesystem: Arc<Storage>) -> Result<()> {
    let error_collection: DocumentErrorCollection = Default::default();

    for file in source_filesystem.read_blog_config_files().await? {
        let config: BlogConfig = match toml::from_str(&file.contents) {
            Ok(config) => config,
            Err(err) => {
                error_collection.register_error(
                    file.relative_path.display().to_string(),
                    anyhow!("Failed to parse blog config file: {err}"),
                );
                continue;
            }
        };

        info!("Found blog config: {:?}", config);

        let path = file.get_stem_path_relative_to(&PathBuf::from("blogs"));
        let blog_name: BlogName = path.into();

        info!("Found blog name: {}", blog_name);
    }

    if error_collection.is_empty() {
        Ok(())
    } else {
        Err(anyhow!("{error_collection}"))
    }
}
