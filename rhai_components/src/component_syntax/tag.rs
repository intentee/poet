use super::attribute::Attribute;
use super::tag_name::TagName;

#[derive(Clone, Debug, Hash)]
pub struct Tag {
    pub attributes: Vec<Attribute>,
    pub is_closing: bool,
    pub is_self_closing: bool,
    pub tag_name: TagName,
}
