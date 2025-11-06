use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::{Error, Result};
use atom_syndication::{Content, Entry, Feed, FixedDateTime, Generator, Link, Person, Text};
use chrono::Utc;
use chrono::{DateTime, NaiveDate};
use log::warn;

use crate::asset_manager::AssetManager;
use crate::content_document_reference::ContentDocumentReference;
use crate::document_error_collection::DocumentErrorCollection;
use crate::filesystem::Filesystem;
use crate::filesystem::memory::Memory;

const POET_WEBSITE_URL: &str = "https://poet.intentee.com";

pub fn eval_content_document_feed(
    parent_document: &ContentDocumentReference,
    document_collection: Vec<ContentDocumentReference>,
    feed_name: String,
    memory_filesystem: Arc<Memory>,
    asset_manager: AssetManager,
    error_collection: &DocumentErrorCollection,
) -> () {
    let mut entries: Vec<Entry> = Vec::new();
    let feed_path = format!(
        "{}/{}",
        parent_document.basename().get_collection_name(),
        feed_name
    );

    for document in document_collection {
        match generate_entry(document.clone(), &asset_manager) {
            Ok(entry) => entries.push(entry),
            Err(err) => error_collection.register_error(feed_path.clone(), err),
        };
    }

    let feed = generate_feed(parent_document.clone(), entries, asset_manager);

    if let Err(err) =
        memory_filesystem.set_file_contents_sync(&Path::new(&feed_path), &feed.to_string())
    {
        error_collection.register_error(feed_name, err);
    }
}

fn generate_feed(
    reference: ContentDocumentReference,
    entries: Vec<Entry>,
    asset_manager: AssetManager,
) -> Feed {
    let image_url = asset_manager
        .file(&reference.clone().front_matter.image)
        .unwrap_or_default();

    let generator = Generator {
        value: "Poet".to_string(),
        uri: Some(POET_WEBSITE_URL.to_string()),
        version: None,
    };

    let authors = vec![Person {
        name: reference.front_matter.author.to_string(),
        email: None,
        uri: None,
    }];

    Feed {
        title: Text::from(reference.clone().front_matter.title),
        id: reference.basename().get_collection_name(),
        updated: FixedDateTime::from(Utc::now()),
        authors,
        generator: Some(generator),
        icon: Some(image_url.clone()),
        logo: Some(image_url),
        entries,
        ..Default::default()
    }
}

fn generate_entry(
    reference: ContentDocumentReference,
    asset_manager: &AssetManager,
) -> Result<Entry, Error> {
    let link = reference
        .canonical_link()
        .map_err(|err| anyhow!("Error while getting canonical link: {err}"))?;

    let date = get_date(reference.clone().front_matter.date, &reference.basename_path)?;

    let mut links = vec![Link {
        href: link.clone(),
        rel: "alternate".to_string(),
        ..Default::default()
    }];

    if !reference.clone().front_matter.image.trim().is_empty() {
        let image_url = asset_manager
            .file(&reference.clone().front_matter.image)
            .unwrap_or_default();

        let mime_type = mime_guess::from_path(&image_url).first_or_octet_stream();

        links.push(Link {
            href: image_url,
            rel: "enclosure".to_string(),
            mime_type: Some(mime_type.to_string()),
            ..Default::default()
        });
    }

    let content = Some(Content {
        value: Some(reference.front_matter.description.clone()),
        content_type: Some("html".to_string()),
        ..Default::default()
    });

    if reference.front_matter.author.clone() == "unknown".to_string() {
        warn!("No author found for document: {:?}", reference.basename_path);
    }

    let authors = vec![Person {
        name: reference.front_matter.author.clone(),
        email: None,
        uri: None,
    }];

    Ok(Entry {
        published: Some(date.into()),
        title: Text::plain(reference.front_matter.title.clone()),
        id: link.clone(),
        updated: date.into(),
        authors,
        content,
        links,
        ..Default::default()
    })
}

fn get_date(date: String, basename_path: &PathBuf) -> Result<DateTime<Utc>, Error> {
    if date.trim().is_empty() {
        warn!("No date found for document: {:?}", basename_path);
        return Ok(Utc::now());
    }

    let naive_date = NaiveDate::parse_from_str(&date, "%d/%m/%Y")
        .map_err(|err| anyhow!("Invalid date format '{}': {err}", date))?;

    let time = naive_date
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| anyhow!("Failed to create timestamp from date '{}'", date))?;

    Ok(DateTime::<Utc>::from_naive_utc_and_offset(time, Utc))
}
