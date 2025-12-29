use rhai::CustomType;
use rhai::TypeBuilder;

use crate::author_basename::AuthorBasename;
use crate::author_data::AuthorData;

#[derive(Clone)]
pub struct Author {
    pub basename: AuthorBasename,
    pub data: AuthorData,
}

impl Author {
    fn rhai_data(&mut self) -> AuthorData {
        self.data.clone()
    }

    fn rhai_basename(&mut self) -> String {
        self.basename.to_string()
    }
}

impl CustomType for Author {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("Author")
            .with_get("basename", Self::rhai_basename)
            .with_get("data", Self::rhai_data);
    }
}
