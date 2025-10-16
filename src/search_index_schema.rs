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

        let basename = schema_builder.add_text_field("basename", STORED | TEXT);
        let title = schema_builder.add_text_field("title", TEXT);
        let description = schema_builder.add_text_field("description", TEXT);
        let header = schema_builder.add_text_field("header", TEXT);
        let paragraph = schema_builder.add_text_field("paragraph", TEXT);

        let schema = schema_builder.build();

        Self {
            fields: SearchIndexFields {
                basename,
                description,
                header,
                paragraph,
                title,
            },
            schema,
        }
    }
}
