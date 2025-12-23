use rhai::CustomType;
use rhai::TypeBuilder;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ArgumentWithInput {
    pub description: String,
    pub input: String,
    pub required: bool,
    pub title: String,
}

impl ArgumentWithInput {
    pub fn rhai_description(&mut self) -> String {
        self.description.clone()
    }

    pub fn rhai_input(&mut self) -> String {
        self.input.clone()
    }

    pub fn rhai_required(&mut self) -> bool {
        self.required
    }

    pub fn rhai_title(&mut self) -> String {
        self.title.clone()
    }
}

impl CustomType for ArgumentWithInput {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("ArgumentWithInput")
            .with_get("description", Self::rhai_description)
            .with_get("input", Self::rhai_input)
            .with_get("required", Self::rhai_required)
            .with_get("title", Self::rhai_title);
    }
}
