use std::path::PathBuf;

use crate::generated_file_kind::GeneratedFileKind;

#[derive(Clone, Debug)]
pub struct GeneratedFile {
    pub contents: String,
    pub kind: GeneratedFileKind,
    pub relative_path: PathBuf,
}
