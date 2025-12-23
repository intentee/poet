use super::attribute_value::AttributeValue;
use crate::SmartStringLazy;

#[derive(Clone, Debug, Hash)]
pub struct Attribute {
    pub name: SmartStringLazy,
    pub value: Option<AttributeValue>,
}
