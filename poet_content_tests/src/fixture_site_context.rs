use std::sync::Arc;

use markdown::mdast::Node;
use markdown::mdast::Root;
use poet_content::author_collection::AuthorCollection;
use poet_content::content_document_index::ContentDocumentIndex;
use poet_content::content_document_linker::ContentDocumentLinker;
use poet_content::content_document_reference::ContentDocumentReference;
use poet_content::content_document_source::ContentDocumentSource;
use poet_content::content_error::ContentError;
use poet_content::content_site_context::ContentSiteContext;
use poet_filesystem::file_entry::FileEntry;
use poet_filesystem::file_entry_stub::FileEntryStub;

pub fn fixture_site_context(
    references: &[ContentDocumentReference],
    available_authors: AuthorCollection,
) -> Result<ContentSiteContext, ContentError> {
    let content_document_sources: Vec<ContentDocumentSource> = references
        .iter()
        .map(|reference| ContentDocumentSource {
            file_entry: FileEntry::from(FileEntryStub {
                contents: String::new(),
                relative_path: reference.basename_path.with_extension("md"),
            }),
            mdast: Node::Root(Root {
                children: vec![],
                position: None,
            }),
            reference: reference.clone(),
        })
        .collect();
    let content_document_index = ContentDocumentIndex::build(&content_document_sources)?;
    let content_document_collections_ranked = content_document_index.rank_collections()?;

    Ok(ContentSiteContext {
        available_authors: Arc::new(available_authors),
        content_document_collections_ranked: Arc::new(content_document_collections_ranked),
        content_document_linker: ContentDocumentLinker {
            content_document_basename_by_id: Arc::new(
                content_document_index.content_document_basename_by_id,
            ),
            content_document_by_basename: Arc::new(
                content_document_index.content_document_by_basename,
            ),
        },
        is_watching: false,
    })
}
