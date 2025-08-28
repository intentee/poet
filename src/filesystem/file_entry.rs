use std::path::PathBuf;

use rhai::CustomType;
use rhai::TypeBuilder;

#[derive(Clone, Debug)]
pub struct FileEntry {
    pub contents: String,
    pub relative_path: PathBuf,
}

impl FileEntry {
    pub fn get_relative_path(&mut self) -> String {
        self.relative_path.to_string_lossy().to_string()
    }

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

    pub fn has_extension(&self, extension: &str) -> bool {
        self.relative_path
            .extension()
            .map(|ext| ext == extension)
            .unwrap_or(false)
    }

    pub fn is_markdown(&self) -> bool {
        self.has_extension("md")
    }

    pub fn is_rhai(&self) -> bool {
        self.has_extension("rhai")
    }
}

impl CustomType for FileEntry {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("FileEntry")
            .with_get("relative_path", Self::get_relative_path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_stem_relative_to() {
        let file_entry = FileEntry {
            contents: String::new(),
            relative_path: PathBuf::from("project/shortcodes/example/foo.rhai"),
        };

        let base = PathBuf::from("project/shortcodes");
        let stem = file_entry.get_stem_relative_to(&base);

        assert_eq!(stem, "example/foo");
    }
}
