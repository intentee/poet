use tantivy::schema::STORED;
use tantivy::schema::Schema;
use tantivy::schema::TEXT;

use crate::search_index_fields::SearchIndexFields;

pub struct SearchIndexSchema {
    pub fields: SearchIndexFields,
    pub schema: Schema,
}

impl SearchIndexSchema {
    pub fn new() -> Self {
        let mut schema_builder = Schema::builder();

        let title = schema_builder.add_text_field("title", STORED | TEXT);
        let description = schema_builder.add_text_field("description", STORED | TEXT);
        let header = schema_builder.add_text_field("header", STORED | TEXT);
        let paragraph = schema_builder.add_text_field("paragraph", STORED | TEXT);

        let schema = schema_builder.build();

        Self {
            fields: SearchIndexFields {
                description,
                header,
                paragraph,
                title,
            },
            schema,
        }
    }
}
