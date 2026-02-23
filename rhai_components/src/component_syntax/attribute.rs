use super::attribute_value::AttributeValue;

#[derive(Clone, Debug, Hash)]
pub struct Attribute {
    pub name: String,
    pub value: Option<AttributeValue>,
}
