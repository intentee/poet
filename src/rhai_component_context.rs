use rhai::CustomType;
use rhai::TypeBuilder;

use crate::front_matter::FrontMatter;

#[derive(Clone)]
pub struct RhaiComponentContext {
    pub front_matter: FrontMatter,
}

impl RhaiComponentContext {
    pub fn get_front_matter(&mut self) -> FrontMatter {
        self.front_matter.clone()
    }
}

impl CustomType for RhaiComponentContext {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("RhaiComponentContext")
            .with_get("front_matter", Self::get_front_matter);
    }
}
