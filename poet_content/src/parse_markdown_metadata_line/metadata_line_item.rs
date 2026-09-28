#[derive(Debug, Eq, PartialEq)]
pub enum MetadataLineItem {
    Flag { name: String },
    Pair { name: String, value: String },
}
