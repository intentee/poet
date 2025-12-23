use super::expression_reference::ExpressionReference;
use super::tag::Tag;
use crate::SmartStringLazy;

#[derive(Clone, Debug, Hash)]
pub enum TagStackNode {
    BodyExpression(ExpressionReference),
    Tag {
        children: Vec<TagStackNode>,
        is_closed: bool,
        opening_tag: Option<Tag>,
    },
    Text(SmartStringLazy),
}
