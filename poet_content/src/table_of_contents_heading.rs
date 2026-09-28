use rhai::CustomType;
use rhai::TypeBuilder;

#[derive(Clone)]
pub struct TableOfContentsHeading {
    pub content: String,
    pub depth: i64,
    pub id: String,
}

impl TableOfContentsHeading {
    fn rhai_content(&mut self) -> String {
        self.content.clone()
    }

    const fn rhai_depth(&mut self) -> i64 {
        self.depth
    }

    fn rhai_id(&mut self) -> String {
        self.id.clone()
    }
}

impl CustomType for TableOfContentsHeading {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("Heading")
            .with_get("content", Self::rhai_content)
            .with_get("depth", Self::rhai_depth)
            .with_get("id", Self::rhai_id);
    }
}
