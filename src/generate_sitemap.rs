use anyhow::{Error, Result};
use chrono::Utc;
use sitemap_rs::url::Url;
use sitemap_rs::url_set::UrlSet;

use crate::content_document_basename::ContentDocumentBasename;
use crate::content_document_reference::ContentDocumentReference;

pub fn generate_sitemap(
    base_url: &String,
    content_document_by_basename: std::collections::hash_map::Values<
        '_,
        ContentDocumentBasename,
        ContentDocumentReference,
    >,
) -> Result<String, Error> {
    let last_modified = Utc::now().fixed_offset();
    let mut urls: Vec<Url> = vec![Url::new(
        base_url.clone(),
        Some(last_modified),
        None,
        Some(0.8),
        None,
        None,
        None,
    )?];

    for reference in content_document_by_basename {
        let mut page_path = reference
            .basename_path
            .to_string_lossy()
            .into_owned()
            .replace("index", "");

        if page_path != "" {
            page_path = format!("{base_url}{page_path}");

            urls.push(Url::new(
                page_path,
                Some(last_modified),
                None,
                Some(0.5),
                None,
                None,
                None,
            )?);
        }
    }

    let url_set: UrlSet = UrlSet::new(urls)?;
    let mut buf: Vec<u8> = Vec::<u8>::new();
    url_set.write(&mut buf).unwrap();

    Ok(String::from_utf8(buf)?)
}
