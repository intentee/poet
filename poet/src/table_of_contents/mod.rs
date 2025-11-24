pub mod heading;

use rhai::Array;
use rhai::CustomType;
use rhai::Dynamic;
use rhai::TypeBuilder;

use crate::table_of_contents::heading::Heading;

#[derive(Clone)]
pub struct TableOfContents {
    pub headings: Vec<Heading>,
}

impl TableOfContents {
    fn rhai_headings(&mut self) -> Array {
        self.headings
            .iter()
            .map(|heading| Dynamic::from(heading.clone()))
            .collect::<_>()
    }
}

impl CustomType for TableOfContents {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("TableOfContents")
            .with_get("headings", Self::rhai_headings);
    }
}
