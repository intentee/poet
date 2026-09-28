use poet_content::author::Author;
use poet_content::author_basename::AuthorBasename;
use poet_content::author_collection::AuthorCollection;
use poet_content::author_data::AuthorData;

#[test]
fn resolves_found_and_missing_authors() {
    let mut author_collection = AuthorCollection::default();

    author_collection.insert(Author {
        basename: AuthorBasename("alice".to_owned()),
        data: AuthorData {
            name: "Alice".to_owned(),
        },
    });

    let resolved_authors = author_collection.resolve(&["alice".to_owned(), "ghost".to_owned()]);

    assert!(matches!(
        resolved_authors.found_authors.as_slice(),
        [Author { basename: AuthorBasename(basename), .. }] if basename == "alice"
    ));
    assert_eq!(resolved_authors.missing_authors, vec!["ghost".to_owned()]);
}
