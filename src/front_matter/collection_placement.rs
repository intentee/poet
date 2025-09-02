use rhai::CustomType;
use rhai::TypeBuilder;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CollectionPlacement {
    #[serde(default)]
    pub after: Option<String>,
    pub name: String,
    #[serde(default)]
    pub parent: Option<String>,
}

impl CollectionPlacement {
    fn rhai_name(&mut self) -> String {
        self.name.clone()
    }
}

impl CustomType for CollectionPlacement {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("CollectionPlacement")
            .with_get("name", Self::rhai_name);
    }
}
