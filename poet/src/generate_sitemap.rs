use std::path::Path;

use anyhow::Result;
use anyhow::anyhow;
use chrono::Utc;
use sitemap_rs::url::Url;
use sitemap_rs::url_set::UrlSet;

use crate::content_document_reference::ContentDocumentReference;

pub fn create_sitemap<'a>(
    content_documents: impl Iterator<Item = &'a ContentDocumentReference>,
) -> Result<String> {
    let last_modified = Utc::now().fixed_offset();
    let mut urls: Vec<Url> = Vec::new();

    for reference in content_documents {
        let url = reference.canonical_link().map_err(|e| anyhow!(e))?;
        let priority = if reference.basename_path == Path::new("index") {
            0.8
        } else {
            0.5
        };

        urls.push(Url::new(
            url,
            Some(last_modified),
            None,
            Some(priority),
            None,
            None,
            None,
        )?);
    }

    let url_set = UrlSet::new(urls)?;
    let mut buf: Vec<u8> = Vec::new();
    url_set.write(&mut buf)?;

    Ok(String::from_utf8(buf)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content_document_front_matter::ContentDocumentFrontMatter;

    fn reference(basename: &str) -> ContentDocumentReference {
        ContentDocumentReference {
            basename_path: basename.into(),
            front_matter: ContentDocumentFrontMatter::mock(basename),
            generated_page_base_path: "https://example.com/".to_string(),
        }
    }

    #[test]
    fn index_document_receives_high_priority() -> Result<()> {
        let references = [reference("index")];
        let sitemap = create_sitemap(references.iter())?;

        assert!(sitemap.contains("https://example.com/"));
        assert!(sitemap.contains("<priority>0.8</priority>"));

        Ok(())
    }

    #[test]
    fn non_index_document_receives_default_priority() -> Result<()> {
        let references = [reference("guide")];
        let sitemap = create_sitemap(references.iter())?;

        assert!(sitemap.contains("https://example.com/guide/"));
        assert!(sitemap.contains("<priority>0.5</priority>"));

        Ok(())
    }
}
