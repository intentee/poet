use std::io;
use std::path::PathBuf;

use poet_assets::asset_error::AssetError;
use poet_filesystem::filesystem_error::FilesystemError;
use poet_mdx::document_error_collection::DocumentErrorCollection;
use poet_mdx::mdx_error::MdxError;
use rhai::EvalAltResult;
use rhai_components::rhai_components_error::RhaiComponentsError;
use sitemap_rs::url_error::UrlError;
use sitemap_rs::url_set_error::UrlSetError;
use thiserror::Error;

use crate::content_document_basename::ContentDocumentBasename;
use crate::date_parse_attempt::DateParseAttempt;

#[derive(Debug, Error)]
pub enum ContentError {
    #[error(
        "document '{basename}' is placed after '{after}', which is not in collection '{collection_name}'"
    )]
    AfterDocumentNotInCollection {
        after: ContentDocumentBasename,
        basename: ContentDocumentBasename,
        collection_name: String,
    },
    #[error("author '{author_name}' of document '{basename}' does not exist")]
    AuthorNotFound {
        author_name: String,
        basename: ContentDocumentBasename,
    },
    #[error("collection '{collection_name}' has a cycle of succeeding documents at '{basename}'")]
    CollectionCycle {
        basename: ContentDocumentBasename,
        collection_name: String,
    },
    #[error("collection '{collection_name}' is not used by any document")]
    CollectionNotUsed { collection_name: String },
    #[error("unable to create sitemap")]
    CreateSitemap(#[source] UrlSetError),
    #[error("unable to create sitemap entry")]
    CreateSitemapUrl(#[source] UrlError),
    #[error("document '{basename}' does not belong to any collection")]
    DocumentHasNoCollection { basename: ContentDocumentBasename },
    #[error("document id '#{id}' is used by both '{first_basename}' and '{second_basename}'")]
    DuplicateDocumentId {
        first_basename: ContentDocumentBasename,
        id: String,
        second_basename: ContentDocumentBasename,
    },
    #[error("unable to evaluate expression '{expression}'")]
    EvaluateExpression {
        expression: String,
        #[source]
        source: RhaiComponentsError,
    },
    #[error("unable to evaluate MDX element")]
    EvaluateMdxElement(#[source] MdxError),
    #[error("unable to highlight code written in '{language}'")]
    HighlightCode {
        language: String,
        #[source]
        source: syntect::Error,
    },
    #[error("document '{basename}' is not used in the hierarchy")]
    HierarchyDocumentNotFound { basename: ContentDocumentBasename },
    #[error("invalid authors\n{0}")]
    InvalidAuthors(DocumentErrorCollection<Self>),
    #[error("unable to parse '{value}' as a date with any of {} known formats", attempts.len())]
    InvalidDate {
        attempts: Vec<DateParseAttempt>,
        value: String,
    },
    #[error("invalid documents\n{0}")]
    InvalidDocuments(DocumentErrorCollection<Self>),
    #[error("no document has id '#{id}'")]
    LinkedDocumentIdNotFound { id: String },
    #[error("linked document '{basename}' does not exist")]
    LinkedDocumentNotFound { basename: ContentDocumentBasename },
    #[error("linked document '{basename}' is not rendered")]
    LinkedDocumentNotRendered { basename: ContentDocumentBasename },
    #[error("content file '{relative_path}' has no front matter")]
    MissingFrontMatter { relative_path: PathBuf },
    #[error("parent document '{parent}' of '{basename}' does not exist")]
    ParentDocumentNotFound {
        basename: ContentDocumentBasename,
        parent: ContentDocumentBasename,
    },
    #[error("unable to parse author file '{relative_path}'")]
    ParseAuthor {
        relative_path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
    #[error("unable to parse code block metadata '{metadata_line}', stopped at '{unparsed}'")]
    ParseCodeMetadata {
        metadata_line: String,
        unparsed: String,
    },
    #[error("unable to parse content file '{relative_path}'")]
    ParseContentDocument {
        relative_path: PathBuf,
        #[source]
        source: MdxError,
    },
    #[error(
        "document '{basename}' declares primary collection '{primary_collection}', but does not belong to it"
    )]
    PrimaryCollectionNotAmongPlacements {
        basename: ContentDocumentBasename,
        primary_collection: String,
    },
    #[error(
        "document '{basename}' belongs to multiple collections, but does not declare its primary collection"
    )]
    PrimaryCollectionNotDeclared { basename: ContentDocumentBasename },
    #[error("unable to read author files")]
    ReadAuthorFiles(#[source] FilesystemError),
    #[error("unable to read content files")]
    ReadContentFiles(#[source] FilesystemError),
    #[error("unable to render layout '{layout}'")]
    RenderLayout {
        layout: String,
        #[source]
        source: RhaiComponentsError,
    },
    #[error("unable to resolve image")]
    ResolveImage(#[source] AssetError),
    #[error("document '{basename}' is placed after '{after}', which does not exist")]
    SucceedingDocumentNotFound {
        after: ContentDocumentBasename,
        basename: ContentDocumentBasename,
    },
    #[error("table of contents is not available while rendering document headings")]
    TableOfContentsUnavailable,
    #[error("unable to write sitemap")]
    WriteSitemap(#[source] io::Error),
}

impl From<ContentError> for Box<EvalAltResult> {
    fn from(content_error: ContentError) -> Self {
        Self::new(EvalAltResult::ErrorSystem(
            String::new(),
            Box::new(content_error),
        ))
    }
}
