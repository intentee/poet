use tantivy::schema::Field;

pub struct SearchIndexFields {
    pub description: Field,
    pub title: Field,
}
