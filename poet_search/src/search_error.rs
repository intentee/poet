use poet_content::content_document_basename::ContentDocumentBasename;
use tantivy::DocAddress;
use tantivy::TantivyError;
use tantivy::query::QueryParserError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SearchError {
    #[error("unable to add content documents to the search index")]
    AddDocuments(#[source] TantivyError),
    #[error("unable to commit the search index")]
    CommitIndex(#[source] TantivyError),
    #[error("unable to create the search index reader")]
    CreateIndexReader(#[source] TantivyError),
    #[error("unable to create the search index writer")]
    CreateIndexWriter(#[source] TantivyError),
    #[error("found document at {document_address:?} has a stored basename that is not text")]
    FoundDocumentBasenameNotText { document_address: DocAddress },
    #[error("found document '{basename}' is not among the indexed content documents")]
    FoundDocumentNotIndexed { basename: ContentDocumentBasename },
    #[error("found document at {document_address:?} has no stored basename")]
    FoundDocumentWithoutBasename { document_address: DocAddress },
    #[error("unable to parse search query '{query}'")]
    ParseQuery {
        query: String,
        #[source]
        source: QueryParserError,
    },
    #[error("unable to read a found document from the search index")]
    ReadFoundDocument(#[source] TantivyError),
    #[error("unable to search the index")]
    Search(#[source] TantivyError),
}
