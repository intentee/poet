use crate::author_basename::AuthorBasename;
use crate::author_front_matter::AuthorFrontMatter;

#[derive(Clone)]
pub struct Author {
    pub basename: AuthorBasename,
    pub front_matter: AuthorFrontMatter,
}
