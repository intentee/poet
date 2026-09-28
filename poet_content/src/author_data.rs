use rhai::CustomType;
use rhai::TypeBuilder;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Hash, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorData {
    pub name: String,
}

impl AuthorData {
    #[cfg(test)]
    pub fn mock(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

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
