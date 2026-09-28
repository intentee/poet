use super::expression_reference::ExpressionReference;
use super::tag_stack_element::TagStackElement;

#[derive(Clone, Debug, Hash)]
pub enum TagStackNode {
    BodyExpression(ExpressionReference),
    Element(TagStackElement),
    Text(String),
}
