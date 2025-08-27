use super::attribute::Attribute;

#[derive(Clone, Debug)]
pub struct Tag {
    pub attributes: Vec<Attribute>,
    pub is_closing: bool,
    pub is_self_closing: bool,
    pub name: String,
}

impl Tag {
    pub fn is_component(&self) -> bool {
        self.name
            .chars()
            .next()
            .map_or(false, |first_character| first_character.is_uppercase())
    }
}
