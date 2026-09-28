use std::path::PathBuf;
use std::sync::Arc;

use poet_content::content_document_reference::ContentDocumentReference;

pub fn fixture_reference(
    basename_path: &str,
    front_matter_toml: &str,
) -> Result<ContentDocumentReference, toml::de::Error> {
    Ok(ContentDocumentReference {
        basename_path: PathBuf::from(basename_path),
        front_matter: Arc::new(toml::from_str(front_matter_toml)?),
        generated_page_base_path: "/".to_owned(),
    })
}
