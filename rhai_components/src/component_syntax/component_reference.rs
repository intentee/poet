use std::borrow::Borrow;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ComponentReference {
    pub name: String,
}

impl Borrow<str> for ComponentReference {
    fn borrow(&self) -> &str {
        &self.name
    }
}
