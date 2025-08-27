use crate::filesystem::file_entry::FileEntry;

pub struct ComponentReference {
    pub file_entry: FileEntry,
    pub global_fn_name: String,
    pub name: String,
    pub path: String,
}
