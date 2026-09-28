use super::tag_stack_node::TagStackNode;

#[derive(Clone, Debug, Hash)]
pub struct TagStack {
    pub children: Vec<TagStackNode>,
}
