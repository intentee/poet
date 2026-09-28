use markdown::message::Message;
use poet_filesystem::filesystem_error::FilesystemError;
use rhai_components::rhai_components_error::RhaiComponentsError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MdxError {
    #[error("unable to build the shortcode renderer")]
    BuildTemplateRenderer(#[source] RhaiComponentsError),
    #[error("unable to evaluate attribute '{attribute_name}' of <{tag_name}>")]
    EvaluateAttribute {
        attribute_name: String,
        tag_name: String,
        #[source]
        source: RhaiComponentsError,
    },
    #[error("attribute expressions are not supported in Markdown, found one in <{tag_name}>")]
    AttributeExpressionNotSupported { tag_name: String },
    #[error("MDX element has no name")]
    ElementWithoutName,
    #[error("unable to parse TOML front matter")]
    ParseFrontMatter(#[source] toml::de::Error),
    #[error("unable to parse Markdown: {message}")]
    ParseMarkdown { message: Message },
    #[error("unable to read shortcode files")]
    ReadShortcodes(#[source] FilesystemError),
    #[error("unable to render component <{tag_name}>")]
    RenderComponent {
        tag_name: String,
        #[source]
        source: RhaiComponentsError,
    },
    #[error("unable to name shortcode component")]
    ResolveShortcodeName(#[source] FilesystemError),
    #[error("void element <{tag_name}> cannot have children")]
    VoidElementWithChildren { tag_name: String },
}
