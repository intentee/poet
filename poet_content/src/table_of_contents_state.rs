use crate::table_of_contents::TableOfContents;

#[derive(Clone)]
pub enum TableOfContentsState {
    BeingCollected,
    Collected(TableOfContents),
}
