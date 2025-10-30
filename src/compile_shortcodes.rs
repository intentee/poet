use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use log::info;

use crate::build_timer::BuildTimer;
use crate::filesystem::Filesystem as _;
use crate::filesystem::storage::Storage;
use crate::rhai_template_factory::RhaiTemplateFactory;
use crate::rhai_template_renderer::RhaiTemplateRenderer;

pub async fn compile_shortcodes(source_filesystem: Arc<Storage>) -> Result<RhaiTemplateRenderer> {
    info!("Compiling shortcodes...");

    let _build_timer = BuildTimer::new();
    let rhai_template_factory = RhaiTemplateFactory::new(
        source_filesystem.base_directory.clone(),
        PathBuf::from("shortcodes"),
    );

    for file in &source_filesystem.read_project_files().await? {
        if file.kind.is_shortcode() {
            rhai_template_factory.register_component_file(file.clone());
        }
    }

    rhai_template_factory.try_into()
}
