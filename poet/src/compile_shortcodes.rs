use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use log::info;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;

use crate::build_timer::BuildTimer;
use crate::filesystem::Filesystem as _;
use crate::filesystem::storage::Storage;
use crate::rhai_template_renderer_factory::RhaiTemplateRendererFactory;

pub async fn compile_shortcodes(source_filesystem: Arc<Storage>) -> Result<RhaiTemplateRenderer> {
    info!("Compiling shortcodes...");

    let _build_timer = BuildTimer::default();
    let rhai_template_factory = RhaiTemplateRendererFactory::new(
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

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use anyhow::Result;
    use rhai::Dynamic;
    use tempfile::TempDir;
    use tempfile::tempdir;

    use super::compile_shortcodes;
    use crate::content_document_component_context::ContentDocumentComponentContext;
    use crate::filesystem::Filesystem as _;
    use crate::filesystem::storage::Storage;

    struct TestStorage {
        _directory: TempDir,
        filesystem: Arc<Storage>,
    }

    async fn storage_with(files: &[(&str, &str)]) -> Result<TestStorage> {
        let directory = tempdir()?;
        let filesystem = Arc::new(Storage {
            base_directory: directory.path().to_path_buf(),
        });

        for (relative_path, contents) in files {
            filesystem
                .set_file_contents(Path::new(relative_path), contents)
                .await?;
        }

        Ok(TestStorage {
            _directory: directory,
            filesystem,
        })
    }

    #[tokio::test]
    async fn compiles_shortcodes_into_a_renderer_that_renders_registered_components() -> Result<()>
    {
        let source_storage = storage_with(&[(
            "shortcodes/PrimaryNavigation.rhai",
            "fn template(context, props, content) { component { <nav>{content}</nav> } }",
        )])
        .await?;

        let renderer = compile_shortcodes(source_storage.filesystem.clone()).await?;

        let rendered = renderer.render(
            "PrimaryNavigation",
            ContentDocumentComponentContext::mock(),
            Dynamic::UNIT,
            Dynamic::from("links".to_string()),
        )?;

        assert!(rendered.contains("<nav>links</nav>"));

        Ok(())
    }

    #[tokio::test]
    async fn errors_when_a_shortcode_has_invalid_syntax() -> Result<()> {
        let source_storage = storage_with(&[(
            "shortcodes/Broken.rhai",
            "fn template(context, props, content) { @ this is not valid rhai @ }",
        )])
        .await?;

        assert!(
            compile_shortcodes(source_storage.filesystem.clone())
                .await
                .is_err()
        );

        Ok(())
    }
}
