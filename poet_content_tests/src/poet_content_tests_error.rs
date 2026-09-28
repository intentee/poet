use std::io;

use esbuild_metafile::error::Error as EsbuildMetafileError;
use poet_content::content_error::ContentError;
use poet_filesystem::filesystem_error::FilesystemError;
use poet_mdx::mdx_error::MdxError;
use rhai::EvalAltResult;
use rhai_components::rhai_components_error::RhaiComponentsError;
use syntect::LoadingError;
use syntect::parsing::ParseSyntaxError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PoetContentTestsError {
    #[error("content operation failed")]
    Content(#[from] ContentError),
    #[error("Rhai script failed")]
    EvaluateScript(#[from] Box<EvalAltResult>),
    #[error("filesystem operation failed")]
    Filesystem(#[from] FilesystemError),
    #[error("unable to load syntax definitions")]
    LoadSyntax(#[from] LoadingError),
    #[error("fixture document '{basename}' does not exist")]
    MissingFixtureDocument { basename: String },
    #[error("MDX operation failed")]
    Mdx(#[from] MdxError),
    #[error("unable to parse syntax definition fixture")]
    ParseSyntaxFixture(#[from] ParseSyntaxError),
    #[error("unable to parse front matter fixture")]
    ParseFrontMatterFixture(#[from] toml::de::Error),
    #[error("unable to parse esbuild metafile fixture")]
    ParseMetafileFixture(#[from] EsbuildMetafileError),
    #[error("unable to prepare test fixture on disk")]
    PrepareFixture(#[from] io::Error),
    #[error("unable to build template renderer")]
    RhaiComponents(#[from] RhaiComponentsError),
}
