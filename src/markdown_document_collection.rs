use crate::markdown_document_in_collection::MarkdownDocumentInCollection;

#[derive(Clone, Debug, Default)]
pub struct MarkdownDocumentCollection {
    pub documents: Vec<MarkdownDocumentInCollection>,
}
