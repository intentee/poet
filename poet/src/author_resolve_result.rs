use crate::author::Author;

pub struct AuthorResolveResult {
    pub found_authors: Vec<Author>,
    pub missing_authors: Vec<String>,
}
