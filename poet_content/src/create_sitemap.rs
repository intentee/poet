use std::path::Path;

use sitemap_rs::jiff::Zoned;
use sitemap_rs::url::Url;
use sitemap_rs::url_set::UrlSet;

use crate::content_document_reference::ContentDocumentReference;
use crate::content_error::ContentError;

const DEFAULT_PAGE_PRIORITY: f32 = 0.5;
const HOME_PAGE_PRIORITY: f32 = 0.8;

fn sitemap_url(
    reference: &ContentDocumentReference,
    last_modified: &Zoned,
) -> Result<Url, ContentError> {
    let priority = if reference.basename_path == Path::new("index") {
        HOME_PAGE_PRIORITY
    } else {
        DEFAULT_PAGE_PRIORITY
    };

    Url::new(
        reference.canonical_link(),
        vec![],
        Some(last_modified.clone()),
        None,
        Some(priority),
        None,
        None,
        None,
    )
    .map_err(ContentError::CreateSitemapUrl)
}

pub fn create_sitemap<'reference>(
    last_modified: &Zoned,
    references: impl Iterator<Item = &'reference ContentDocumentReference>,
) -> Result<String, ContentError> {
    references
        .map(|reference| sitemap_url(reference, last_modified))
        .collect::<Result<Vec<Url>, ContentError>>()
        .and_then(|urls| UrlSet::new(urls).map_err(ContentError::CreateSitemap))
        .and_then(|url_set| {
            let mut sitemap_bytes: Vec<u8> = vec![];

            url_set
                .write(&mut sitemap_bytes)
                .map(|()| String::from_utf8_lossy(&sitemap_bytes).into_owned())
                .map_err(ContentError::WriteSitemap)
        })
}
