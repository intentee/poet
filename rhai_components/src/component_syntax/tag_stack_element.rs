use super::tag::Tag;
use super::tag_stack_node::TagStackNode;

#[derive(Clone, Debug, Hash)]
pub struct TagStackElement {
    pub children: Vec<TagStackNode>,
    pub opening_tag: Tag,
}
