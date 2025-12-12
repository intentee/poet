type SmartString = smartstring::SmartString<smartstring::LazyCompact>;

#[derive(Clone)]
pub struct ComponentReference {
    pub name: SmartString,
    pub path: SmartString,
}
