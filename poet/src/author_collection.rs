use std::collections::BTreeMap;

use rhai::CustomType;
use rhai::TypeBuilder;

use crate::author::Author;
use crate::author_basename::AuthorBasename;

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

    pub fn resolve(&self, names: &[String]) -> (Vec<Author>, Vec<String>) {
        let mut authors = Vec::new();
        let mut not_found = Vec::new();

        for name in names {
            let basename = AuthorBasename::from(name.clone());

            match self.authors.get(&basename) {
                Some(author) => authors.push(author.clone()),
                None => not_found.push(name.clone()),
            }
        }

        (authors, not_found)
    }
}

impl CustomType for AuthorCollection {
    fn build(mut builder: TypeBuilder<Self>) {
        builder.with_name("AuthorCollection");
    }
}
