use std::io;

use esbuild_metafile::error::Error as EsbuildMetafileError;
use poet_content::content_error::ContentError;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use poet_mdx::mdx_error::MdxError;
use poet_prompt::prompt_error::PromptError;
use rhai_components::rhai_components_error::RhaiComponentsError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PoetPromptTestsError {
    #[error("unable to build the fixture site context")]
    Content(#[source] Box<ContentError>),
    #[error("unable to prepare the fixture project")]
    FixtureProject(#[source] Box<PoetContentTestsError>),
    #[error("MDX operation failed")]
    Mdx(#[from] MdxError),
    #[error("unable to parse front matter fixture")]
    ParseFrontMatterFixture(#[from] toml::de::Error),
    #[error("unable to parse esbuild metafile fixture")]
    ParseMetafileFixture(#[from] EsbuildMetafileError),
    #[error("unable to prepare test fixture on disk")]
    PrepareFixture(#[from] io::Error),
    #[error("prompt operation failed")]
    Prompt(#[source] Box<PromptError>),
    #[error("unable to build template renderer")]
    RhaiComponents(#[from] RhaiComponentsError),
}

impl From<ContentError> for PoetPromptTestsError {
    fn from(content_error: ContentError) -> Self {
        Self::Content(Box::new(content_error))
    }
}

impl From<PoetContentTestsError> for PoetPromptTestsError {
    fn from(poet_content_tests_error: PoetContentTestsError) -> Self {
        Self::FixtureProject(Box::new(poet_content_tests_error))
    }
}

impl From<PromptError> for PoetPromptTestsError {
    fn from(prompt_error: PromptError) -> Self {
        Self::Prompt(Box::new(prompt_error))
    }
}
