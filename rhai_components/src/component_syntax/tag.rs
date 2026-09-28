use super::attribute::Attribute;
use super::tag_kind::TagKind;
use super::tag_name::TagName;

#[derive(Clone, Debug, Hash)]
pub struct Tag {
    pub attributes: Vec<Attribute>,
    pub kind: TagKind,
    pub tag_name: TagName,
}
