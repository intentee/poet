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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content_document_front_matter::ContentDocumentFrontMatter;

    fn reference(basename: &str, render: bool) -> ContentDocumentReference {
        let mut front_matter = ContentDocumentFrontMatter::mock(basename);

        front_matter.render = render;

        ContentDocumentReference {
            basename_path: basename.into(),
            front_matter,
            generated_page_base_path: "/".to_string(),
        }
    }

    fn linker() -> ContentDocumentLinker {
        let mut content_document_by_basename: HashMap<
            ContentDocumentBasename,
            ContentDocumentReference,
        > = HashMap::new();

        content_document_by_basename.insert("guide".to_string().into(), reference("guide", true));
        content_document_by_basename.insert("draft".to_string().into(), reference("draft", false));

        let mut content_document_basename_by_id: HashMap<String, ContentDocumentBasename> =
            HashMap::new();

        content_document_basename_by_id.insert("guide-id".to_string(), "guide".to_string().into());

        ContentDocumentLinker {
            content_document_basename_by_id: Arc::new(content_document_basename_by_id),
            content_document_by_basename: Arc::new(content_document_by_basename),
        }
    }

    #[test]
    fn resolve_id_returns_basename_for_known_id() {
        assert_eq!(
            linker().resolve_id("#guide-id"),
            Ok("guide".to_string().into())
        );
    }

    #[test]
    fn resolve_id_fails_for_unknown_id() {
        assert!(linker().resolve_id("#missing").is_err());
    }

    #[test]
    fn resolve_id_returns_plain_path_as_basename() {
        assert_eq!(linker().resolve_id("guide"), Ok("guide".to_string().into()));
    }

    #[test]
    fn link_to_returns_canonical_link_for_renderable_document() {
        assert_eq!(linker().link_to("guide"), Ok("/guide/".to_string()));
    }

    #[test]
    fn link_to_fails_for_non_renderable_document() {
        assert!(linker().link_to("draft").is_err());
    }

    #[test]
    fn link_to_fails_for_missing_document() {
        assert!(linker().link_to("ghost").is_err());
    }
}
