use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListResourcesCursorToken {
    pub offset: usize,
    pub per_page: usize,
}
