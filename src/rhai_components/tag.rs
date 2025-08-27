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
            .is_some_and(|first_character| first_character.is_uppercase())
    }

    pub fn is_void_element(&self) -> bool {
        self.name == "!DOCTYPE"
            || self.name == "area"
            || self.name == "base"
            || self.name == "br"
            || self.name == "col"
            || self.name == "embed"
            || self.name == "hr"
            || self.name == "img"
            || self.name == "input"
            || self.name == "link"
            || self.name == "meta"
            || self.name == "param"
            || self.name == "source"
            || self.name == "track"
            || self.name == "wbr"
    }
}
