use std::path::PathBuf;

use anyhow::Result;
use anyhow::anyhow;
use log::info;
use rhai::Dynamic;

use crate::filesystem::Filesystem;
use crate::filesystem::memory::Memory;
use crate::filesystem::storage::Storage;
use crate::rhai_component_context::RhaiComponentContext;
use crate::rhai_template_factory::RhaiTemplateFactory;
use crate::rhai_template_renderer::RhaiTemplateRenderer;

pub async fn build_project(source_filesystem: &Storage) -> Result<Memory> {
    let files = source_filesystem.read_project_files().await?;
    let shortcodes_directory = PathBuf::from("shortcodes");
    let rhai_template_factory = RhaiTemplateFactory::new(
        source_filesystem.base_directory.clone(),
        shortcodes_directory,
    );

    // First pass, process Rhai files to be used as shortcodes or layouts
    for file in &files {
        if file.is_rhai() {
            info!("Processing shortcode file: {:?}", file.relative_path);

            rhai_template_factory.register_component_file(file.clone());
        }
    }

    info!("Compiling the templates...");

    let rhai_template_renderer: RhaiTemplateRenderer = rhai_template_factory.try_into()?;

    println!(
        "{}",
        rhai_template_renderer.render(
            "Test",
            RhaiComponentContext::default(),
            Dynamic::from_map({
                let mut props = rhai::Map::new();

                props.insert("type".into(), "info".into());

                props
            }),
            Dynamic::from("XD"),
        )?
    );

    // for file in &files {
    //     if file.is_markdown() {
    //         info!("Processing content file: {:?}", file.relative_path);
    //
    //         let mdast = string_to_mdast(&file.contents)?;
    //
    //         mdast_to_document(mdast);
    //     }
    // }

    Err(anyhow!("Not implemented yet"))
}
