use std::path::PathBuf;

use anyhow::Result;
use anyhow::anyhow;
use log::info;

use crate::filesystem::Filesystem;
use crate::filesystem::memory::Memory;
use crate::filesystem::storage::Storage;
use crate::mdast_to_document::mdast_to_document;
use crate::rhai_context::RhaiContext;
use crate::shortcode_collection::ShortcodeCollection;
use crate::string_to_mdast::string_to_mdast;

pub async fn build_project(source_filesystem: &Storage) -> Result<Memory> {
    let files = source_filesystem.read_project_files().await?;
    let shortcodes_directory = PathBuf::from("shortcodes");
    let rhai_context =
        RhaiContext::new(source_filesystem.base_directory.join(&shortcodes_directory));
    let mut shortcodes_collection = ShortcodeCollection::default();

    // First pass, process Rhai files to be used as shortcodes or layouts
    for file in &files {
        if file.is_rhai() {
            info!("Processing shortcode file: {:?}", file.relative_path);

            shortcodes_collection.shortcodes.insert(
                file.get_stem_relative_to(&shortcodes_directory),
                rhai_context.compile_shortcode_file(&file)?,
            );
        }
    }

    for file in &files {
        if file.is_markdown() {
            info!("Processing content file: {:?}", file.relative_path);

            let mdast = string_to_mdast(&file.contents)?;

            mdast_to_document(mdast);
        }
    }

    Err(anyhow!("Not implemented yet"))
}
