use rhai::Array;
use rhai::CustomType;
use rhai::Dynamic;
use rhai::TypeBuilder;

use crate::table_of_contents_heading::TableOfContentsHeading;

#[derive(Clone)]
pub struct TableOfContents {
    pub headings: Vec<TableOfContentsHeading>,
}

impl TableOfContents {
    fn rhai_headings(&mut self) -> Array {
        self.headings.iter().cloned().map(Dynamic::from).collect()
    }
}

impl CustomType for TableOfContents {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("TableOfContents")
            .with_get("headings", Self::rhai_headings);
    }
}
