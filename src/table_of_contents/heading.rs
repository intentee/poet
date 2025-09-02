use rhai::CustomType;
use rhai::TypeBuilder;

#[derive(Clone)]
pub struct Heading {
    pub content: String,
    pub depth: i64,
    pub id: String,
}

impl Heading {
    fn rhai_content(&mut self) -> String {
        self.content.clone()
    }

    fn rhai_depth(&mut self) -> i64 {
        self.depth
    }

    fn rhai_id(&mut self) -> String {
        self.id.clone()
    }
}

impl CustomType for Heading {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("Heading")
            .with_get("content", Self::rhai_content)
            .with_get("depth", Self::rhai_depth)
            .with_get("id", Self::rhai_id);
    }
}
