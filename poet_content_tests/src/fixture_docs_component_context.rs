use std::path::Path;

use poet_content::author::Author;
use poet_content::author_basename::AuthorBasename;
use poet_content::author_collection::AuthorCollection;
use poet_content::author_data::AuthorData;
use poet_content::content_document_component_context::ContentDocumentComponentContext;

use crate::fixture_component_context::fixture_component_context;
use crate::fixture_docs_references::fixture_docs_references;
use crate::fixture_site_context::fixture_site_context;
use crate::poet_content_tests_error::PoetContentTestsError;

fn fixture_author(basename: &str, name: &str) -> Author {
    Author {
        basename: AuthorBasename(basename.to_owned()),
        data: AuthorData {
            name: name.to_owned(),
        },
    }
}

pub fn fixture_docs_component_context(
    basename: &str,
) -> Result<ContentDocumentComponentContext, PoetContentTestsError> {
    let references = fixture_docs_references()?;
    let reference = references
        .iter()
        .find(|reference| reference.basename_path == Path::new(basename))
        .cloned()
        .ok_or_else(|| PoetContentTestsError::MissingFixtureDocument {
            basename: basename.to_owned(),
        })?;
    let mut available_authors = AuthorCollection::default();

    available_authors.insert(fixture_author("alice", "Alice"));
    available_authors.insert(fixture_author("bob", "Bob"));

    fixture_component_context(
        vec![fixture_author("alice", "Alice")],
        reference,
        fixture_site_context(&references, available_authors)?,
    )
}
