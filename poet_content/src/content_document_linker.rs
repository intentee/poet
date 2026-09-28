use std::collections::HashMap;
use std::sync::Arc;

use poet_assets::is_external_link::is_external_link;

use crate::content_document_basename::ContentDocumentBasename;
use crate::content_document_reference::ContentDocumentReference;
use crate::content_error::ContentError;
use crate::content_link_target::ContentLinkTarget;

#[derive(Clone, Default)]
pub struct ContentDocumentLinker {
    pub content_document_basename_by_id: Arc<HashMap<String, ContentDocumentBasename>>,
    pub content_document_by_basename:
        Arc<HashMap<ContentDocumentBasename, ContentDocumentReference>>,
}

impl ContentDocumentLinker {
    pub fn link_to(&self, link: &str) -> Result<String, ContentError> {
        let basename = self.resolve_basename(link)?;
        let reference = self
            .content_document_by_basename
            .get(&basename)
            .ok_or_else(|| ContentError::LinkedDocumentNotFound {
                basename: basename.clone(),
            })?;

        if reference.front_matter.render {
            Ok(reference.canonical_link())
        } else {
            Err(ContentError::LinkedDocumentNotRendered { basename })
        }
    }

    pub fn resolve_basename(&self, link: &str) -> Result<ContentDocumentBasename, ContentError> {
        match ContentLinkTarget::from(link) {
            ContentLinkTarget::DocumentId(id) => self
                .content_document_basename_by_id
                .get(&id)
                .cloned()
                .ok_or(ContentError::LinkedDocumentIdNotFound { id }),
            ContentLinkTarget::DocumentPath(basename) => Ok(basename),
        }
    }

    pub fn resolve_link(&self, url: &str) -> Result<String, ContentError> {
        if is_external_link(url) {
            Ok(url.to_owned())
        } else {
            self.link_to(url)
        }
    }
}
