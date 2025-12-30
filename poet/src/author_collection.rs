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
