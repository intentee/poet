use std::path::Path;

use anyhow::Result;
use anyhow::anyhow;
use sitemap_rs::jiff::Timestamp;
use sitemap_rs::jiff::tz::TimeZone;
use sitemap_rs::url::Url;
use sitemap_rs::url_set::UrlSet;

use crate::content_document_reference::ContentDocumentReference;

pub fn create_sitemap<'reference>(
    content_documents: impl Iterator<Item = &'reference ContentDocumentReference>,
) -> Result<String> {
    let last_modified = Timestamp::now().to_zoned(TimeZone::UTC);
    let mut references: Vec<&ContentDocumentReference> = content_documents.collect();
    let mut urls: Vec<Url> = Vec::new();

    references.sort_by(|left, right| left.basename_path.cmp(&right.basename_path));

    for reference in references {
        let url = reference.canonical_link().map_err(|error| anyhow!(error))?;
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
    fn lists_documents_in_basename_order() -> Result<()> {
        let references = [reference("guide"), reference("index"), reference("about")];
        let sitemap = create_sitemap(references.iter())?;

        let about_position = sitemap.find("<loc>https://example.com/about/</loc>");
        let guide_position = sitemap.find("<loc>https://example.com/guide/</loc>");
        let index_position = sitemap.find("<loc>https://example.com/</loc>");

        assert!(about_position.is_some());
        assert!(about_position < guide_position);
        assert!(guide_position < index_position);

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
