use std::collections::HashMap;
use std::sync::Arc;

use crate::content_document_basename::ContentDocumentBasename;
use crate::content_document_reference::ContentDocumentReference;

#[derive(Clone, Default)]
pub struct ContentDocumentLinker {
    pub content_document_basename_by_id: Arc<HashMap<String, ContentDocumentBasename>>,
    pub content_document_by_basename:
        Arc<HashMap<ContentDocumentBasename, ContentDocumentReference>>,
}

impl ContentDocumentLinker {
    pub fn link_to(&self, path: &str) -> Result<String, String> {
        let basename = self.resolve_id(path)?;

        if let Some(reference) = self.content_document_by_basename.get(&basename) {
            if !reference.front_matter.render {
                return Err(format!(
                    "Document cannot be linked to, because rendering of it is disabled: {basename}"
                ));
            }

            match reference.canonical_link() {
                Ok(canonical_link) => Ok(canonical_link),
                Err(err) => Err(format!(
                    "Unable to generate canonical link for {basename}: {err}"
                )),
            }
        } else {
            Err(format!("Document does not exist: {path}"))
        }
    }

    pub fn resolve_id(&self, path: &str) -> Result<ContentDocumentBasename, String> {
        if path.starts_with("#") {
            if let Some(basename) =
                self.content_document_basename_by_id
                    .get(match path.strip_prefix('#') {
                        Some(id) => id,
                        None => return Err("Unable to strip prefix from document id".into()),
                    })
            {
                Ok(basename.clone())
            } else {
                Err(format!("Document with id does not exist: {path}"))
            }
        } else {
            Ok(path.to_string().into())
        }
    }
}
