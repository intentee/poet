use rhai::CustomType;
use rhai::TypeBuilder;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorData {
    pub name: String,
}

impl AuthorData {
    fn rhai_name(&mut self) -> String {
        self.name.clone()
    }
}

impl CustomType for AuthorData {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("AuthorData")
            .with_get("name", Self::rhai_name);
    }
}
