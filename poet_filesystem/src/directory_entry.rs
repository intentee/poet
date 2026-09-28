use std::ffi::OsString;

pub struct DirectoryEntry {
    pub file_name: OsString,
    pub is_directory: bool,
}
