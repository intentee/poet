use tantivy::schema::Field;

pub struct SearchIndexFields {
    pub basename: Field,
    pub description: Field,
    pub header: Field,
    pub paragraph: Field,
    pub title: Field,
}
