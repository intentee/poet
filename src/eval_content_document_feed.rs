use std::path::Path;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::{Error, Result};
use atom_syndication::Content;
use atom_syndication::Entry;
use atom_syndication::Feed;
use atom_syndication::FixedDateTime;
use atom_syndication::Generator;
use atom_syndication::Link;
use atom_syndication::Person;
use atom_syndication::Text;
use chrono::Utc;
use chrono::{DateTime, NaiveDate};

use crate::asset_manager::AssetManager;
use crate::content_document_reference::ContentDocumentReference;
use crate::document_error_collection::DocumentErrorCollection;
use crate::filesystem::Filesystem;
use crate::filesystem::memory::Memory;

const POET_WEBSITE_URL: &str = "https://poet.intentee.com";

pub fn eval_content_document_feed(
    document_collection: Vec<ContentDocumentReference>,
    feed_name: String,
    memory_filesystem: Arc<Memory>,
    asset_manager: AssetManager,
    error_collection: &DocumentErrorCollection,
) -> () {
    let entries: Vec<Entry> = document_collection
        .iter()
        .filter_map(|reference| Some(generate_entry(reference.clone()).ok()?))
        .collect();

    if let Some(reference) = document_collection.first() {
        let feed = generate_feed(reference.clone(), entries, asset_manager);

        if let Err(err) = memory_filesystem.set_file_contents_sync(
            &Path::new(&format!(
                "{}/{}",
                reference.basename().get_collection_name(),
                feed_name
            )),
            &feed.to_string(),
        ) {
            error_collection.register_error(feed_name, err);
        }
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

    Feed {
        title: Text::from(reference.clone().front_matter.title),
        id: reference.basename().get_collection_name(),
        updated: FixedDateTime::from(Utc::now()),
        authors: vec![Person {
            name: "Feed Generator".to_string(),
            email: None,
            uri: None,
        }],
        generator: Some(generator),
        icon: Some(image_url.clone()),
        logo: Some(image_url),
        entries,
        ..Default::default()
    }
}

fn generate_entry(reference: ContentDocumentReference) -> Result<Entry, Error> {
    let link = reference
        .canonical_link()
        .map_err(|err| anyhow!("Error while getting canonical link: {err}"))?;

    let naive_date =
        NaiveDate::parse_from_str(&reference.front_matter.date, "%d/%m/%Y").map_err(|err| {
            anyhow!(
                "Invalid date format '{}': {err}",
                reference.front_matter.date
            )
        })?;

    let date =
        DateTime::<Utc>::from_naive_utc_and_offset(naive_date.and_hms_opt(0, 0, 0).unwrap(), Utc);

    let links = vec![Link {
        href: link.clone(),
        rel: "alternate".to_string(),
        ..Default::default()
    }];

    let content = Some(Content {
        value: Some(reference.front_matter.description.clone()),
        content_type: Some("html".to_string()),
        ..Default::default()
    });

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
