use super::attribute_value::AttributeValue;

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub value: Option<AttributeValue>,
}
