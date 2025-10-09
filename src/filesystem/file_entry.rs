use std::path::PathBuf;

use anyhow::Result;
use anyhow::anyhow;
use rhai::CustomType;
use rhai::TypeBuilder;

use crate::filesystem::file_entry_kind::FileEntryKind;
use crate::filesystem::file_entry_stub::FileEntryStub;

#[derive(Clone, Debug)]
pub struct FileEntry {
    pub contents: String,
    pub kind: FileEntryKind,
    pub relative_path: PathBuf,
}

impl FileEntry {
    pub fn get_stem_path_relative_to(&self, base: &PathBuf) -> PathBuf {
        self.relative_path
            .strip_prefix(base)
            .unwrap_or(&self.relative_path)
            .with_extension("")
    }

    pub fn get_stem_relative_to(&self, base: &PathBuf) -> String {
        self.get_stem_path_relative_to(base)
            .to_string_lossy()
            .to_string()
    }

    fn rhai_relative_path(&mut self) -> String {
        self.relative_path.to_string_lossy().to_string()
    }
}

impl CustomType for FileEntry {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("FileEntry")
            .with_get("relative_path", Self::rhai_relative_path);
    }
}

impl TryFrom<FileEntryStub> for FileEntry {
    type Error = anyhow::Error;

    fn try_from(file_entry_stub: FileEntryStub) -> Result<Self> {
        let top_directory: String = file_entry_stub
            .top_directory()
            .ok_or_else(|| anyhow!("Unable to find file's top directory"))?;
        let extension: String = file_entry_stub
            .extension()
            .ok_or_else(|| anyhow!("Unable to find file's extension"))?;

        Ok(Self {
            contents: file_entry_stub.contents,
            kind: match (top_directory.as_str(), extension.as_str()) {
                ("content", "md") => FileEntryKind::Content,
                ("prompts", "md") => FileEntryKind::Prompt,
                ("shortcodes", "rhai") => FileEntryKind::Shortcode,
                _ => {
                    return Err(anyhow!(
                        "Unable to figure out file kind for: {}",
                        file_entry_stub.relative_path.display()
                    ));
                }
            },
            relative_path: file_entry_stub.relative_path,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_stem_relative_to() {
        let file_entry: FileEntry = FileEntryStub {
            contents: String::new(),
            relative_path: PathBuf::from("shortcodes/example/foo.rhai"),
        }
        .try_into()
        .unwrap();

        let base = PathBuf::from("shortcodes");
        let stem = file_entry.get_stem_relative_to(&base);

        assert_eq!(stem, "example/foo");
    }
}
