use std::collections::BTreeMap;

use rhai::CustomType;
use rhai::TypeBuilder;

use crate::author::Author;
use crate::author_basename::AuthorBasename;
use crate::author_resolve_result::AuthorResolveResult;

#[derive(Clone, Default)]
pub struct AuthorCollection {
    authors: BTreeMap<AuthorBasename, Author>,
}

impl AuthorCollection {
    pub fn insert(&mut self, basename: AuthorBasename, author: Author) {
        self.authors.insert(basename, author);
    }

    pub fn values(&self) -> impl Iterator<Item = &Author> {
        self.authors.values()
    }

    pub fn resolve(&self, names: &[String]) -> AuthorResolveResult {
        let mut found_authors = Vec::new();
        let mut missing_authors = Vec::new();

        for name in names {
            let basename = AuthorBasename::from(name.clone());

            match self.authors.get(&basename) {
                Some(author) => found_authors.push(author.clone()),
                None => missing_authors.push(name.clone()),
            }
        }

        AuthorResolveResult {
            found_authors,
            missing_authors,
        }
    }
}

impl CustomType for AuthorCollection {
    fn build(mut builder: TypeBuilder<Self>) {
        builder.with_name("AuthorCollection");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::author_data::AuthorData;

    fn author(basename: &str) -> Author {
        Author {
            basename: basename.to_string().into(),
            data: AuthorData::mock(basename),
        }
    }

    fn collection() -> AuthorCollection {
        let mut collection = AuthorCollection::default();

        collection.insert("alice".to_string().into(), author("alice"));
        collection.insert("bob".to_string().into(), author("bob"));

        collection
    }

    #[test]
    fn resolve_separates_found_and_missing_authors() {
        let result = collection().resolve(&["alice".to_string(), "carol".to_string()]);

        assert_eq!(result.found_authors.len(), 1);
        assert_eq!(result.found_authors[0].basename, "alice".to_string().into());
        assert_eq!(result.missing_authors, vec!["carol".to_string()]);
    }

    #[test]
    fn resolve_returns_empty_results_for_empty_input() {
        let result = collection().resolve(&[]);

        assert!(result.found_authors.is_empty());
        assert!(result.missing_authors.is_empty());
    }
}
