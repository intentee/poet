use std::io;
use std::path::Path;
use std::sync::Arc;

use poet_assets::asset_path_renderer::AssetPathRenderer;
use poet_content::build_authors::build_authors;
use poet_content::build_project::build_project;
use poet_content::build_project_params::BuildProjectParams;
use poet_content::build_project_result_stub::BuildProjectResultStub;
use poet_content::register_content_rhai_types::register_content_rhai_types;
use poet_filesystem::filesystem::Filesystem as _;
use poet_filesystem::storage::Storage;
use poet_mdx::compile_shortcodes::compile_shortcodes;
use poet_mdx::compile_shortcodes_params::CompileShortcodesParams;
use tempfile::TempDir;
use tempfile::tempdir;

use crate::poet_content_tests_error::PoetContentTestsError;

pub struct FixtureProject {
    pub directory: TempDir,
    pub storage: Storage,
}

impl FixtureProject {
    pub fn create() -> io::Result<Self> {
        let directory = tempdir()?;

        Ok(Self {
            storage: Storage {
                base_directory: directory.path().to_path_buf(),
            },
            directory,
        })
    }

    pub async fn add_file(
        &self,
        relative_path: &str,
        contents: &str,
    ) -> Result<(), PoetContentTestsError> {
        Ok(self
            .storage
            .set_file_contents(Path::new(relative_path), contents)
            .await?)
    }

    pub async fn build(
        &self,
        generate_sitemap: bool,
    ) -> Result<BuildProjectResultStub, PoetContentTestsError> {
        let rhai_template_renderer = compile_shortcodes(CompileShortcodesParams {
            register_rhai_types: register_content_rhai_types,
            source_filesystem: &self.storage,
        })
        .await?;
        let authors = build_authors(&self.storage).await?;

        Ok(build_project(BuildProjectParams {
            asset_path_renderer: AssetPathRenderer {
                base_path: "/".to_owned(),
            },
            authors,
            esbuild_metafile: Arc::default(),
            generate_sitemap,
            generated_page_base_path: "/".to_owned(),
            is_watching: false,
            rhai_template_renderer,
            source_filesystem: &self.storage,
        })
        .await?)
    }
}
