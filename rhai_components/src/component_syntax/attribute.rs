use super::attribute_value::AttributeValue;

type SmartString = smartstring::SmartString<smartstring::LazyCompact>;

#[derive(Clone, Debug, Hash)]
pub struct Attribute {
    pub name: SmartString,
    pub value: Option<AttributeValue>,
}
