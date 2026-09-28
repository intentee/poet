use poet_assets::asset_error::AssetError;
use poet_content::content_error::ContentError;
use poet_filesystem::filesystem_error::FilesystemError;
use poet_mdx::document_error_collection::DocumentErrorCollection;
use poet_mdx::mdx_error::MdxError;
use rhai::EvalAltResult;
use rhai_components::rhai_components_error::RhaiComponentsError;
use serde::de::value::Error as DeserializationError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PromptError {
    #[error(
        "content '{content}' does not belong to any message, start its paragraph with a role marker such as '**user**:'"
    )]
    ContentWithoutRoleMarker { content: String },
    #[error("unable to evaluate expression '{expression}'")]
    EvaluateExpression {
        expression: String,
        #[source]
        source: RhaiComponentsError,
    },
    #[error("unable to evaluate MDX element")]
    EvaluateMdxElement(#[source] MdxError),
    #[error("prompt file name is not valid UTF-8")]
    InvalidPromptName(#[source] FilesystemError),
    #[error("invalid prompt documents\n{0}")]
    InvalidPromptDocuments(DocumentErrorCollection<Self>),
    #[error("message '{content}' has no role, start it with a role marker such as '**user**:'")]
    MessageWithoutRole { content: String },
    #[error("no input provided for argument '{name}'")]
    MissingArgument { name: String },
    #[error("prompt document has no front matter")]
    MissingFrontMatter,
    #[error("unable to parse prompt document")]
    ParsePromptDocument(#[source] MdxError),
    #[error("unable to read prompt files")]
    ReadPromptFiles(#[source] FilesystemError),
    #[error("unable to resolve image '{url}'")]
    ResolveImage {
        url: String,
        #[source]
        source: AssetError,
    },
    #[error("unable to resolve link")]
    ResolveLink(#[source] ContentError),
    #[error("unknown role '{role_name}', you can only use 'assistant' or 'user'")]
    UnknownRole {
        role_name: String,
        #[source]
        source: DeserializationError,
    },
}

impl From<PromptError> for Box<EvalAltResult> {
    fn from(prompt_error: PromptError) -> Self {
        Self::new(EvalAltResult::ErrorSystem(
            String::new(),
            Box::new(prompt_error),
        ))
    }
}
