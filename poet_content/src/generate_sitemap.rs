use std::path::Path;

use anyhow::Result;
use anyhow::anyhow;
use sitemap_rs::jiff::Timestamp;
use sitemap_rs::jiff::tz::TimeZone;
use sitemap_rs::url::Url;
use sitemap_rs::url_set::UrlSet;

use crate::content_document_reference::ContentDocumentReference;

pub fn create_sitemap<'content_document>(
    content_documents: impl Iterator<Item = &'content_document ContentDocumentReference>,
) -> Result<String> {
    let last_modified = Timestamp::now().to_zoned(TimeZone::UTC);
    let mut urls: Vec<Url> = Vec::new();

    for reference in content_documents {
        let url = reference
            .canonical_link()
            .map_err(|canonical_link_error| anyhow!(canonical_link_error))?;
        let priority = if reference.basename_path == Path::new("index") {
            0.8
        } else {
            0.5
        };

        urls.push(Url::new(
            url,
            Vec::new(),
            Some(last_modified.clone()),
            None,
            Some(priority),
            None,
            None,
            None,
        )?);
    }

    let url_set = UrlSet::new(urls)?;
    let mut sitemap_bytes: Vec<u8> = Vec::new();
    url_set.write(&mut sitemap_bytes)?;

    Ok(String::from_utf8(sitemap_bytes)?)
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
