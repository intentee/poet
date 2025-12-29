use crate::author_basename::AuthorBasename;
use crate::author_data::AuthorData;

#[derive(Clone)]
pub struct Author {
    pub basename: AuthorBasename,
    pub data: AuthorData,
}
