use rhai::CustomType;
use rhai::TypeBuilder;

#[derive(Clone)]
pub struct PromptDocumentArgumentWithInput {
    pub description: String,
    pub input: String,
    pub required: bool,
    pub title: String,
}

impl PromptDocumentArgumentWithInput {
    fn rhai_description(&mut self) -> String {
        self.description.clone()
    }

    fn rhai_input(&mut self) -> String {
        self.input.clone()
    }

    const fn rhai_required(&mut self) -> bool {
        self.required
    }

    fn rhai_title(&mut self) -> String {
        self.title.clone()
    }
}

impl CustomType for PromptDocumentArgumentWithInput {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("ArgumentWithInput")
            .with_get("description", Self::rhai_description)
            .with_get("input", Self::rhai_input)
            .with_get("required", Self::rhai_required)
            .with_get("title", Self::rhai_title);
    }
}
