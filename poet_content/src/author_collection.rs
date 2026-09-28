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
    pub fn insert(&mut self, author: Author) {
        self.authors.insert(author.basename.clone(), author);
    }

    #[must_use]
    pub fn resolve(&self, author_names: &[String]) -> AuthorResolveResult {
        let mut found_authors = vec![];
        let mut missing_authors = vec![];

        for author_name in author_names {
            match self.authors.get(&AuthorBasename(author_name.clone())) {
                Some(author) => found_authors.push(author.clone()),
                None => missing_authors.push(author_name.clone()),
            }
        }

        AuthorResolveResult {
            found_authors,
            missing_authors,
        }
    }

    pub fn values(&self) -> impl Iterator<Item = &Author> {
        self.authors.values()
    }
}

impl CustomType for AuthorCollection {
    fn build(mut builder: TypeBuilder<Self>) {
        builder.with_name("AuthorCollection");
    }
}
