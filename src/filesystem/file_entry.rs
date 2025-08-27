use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct FileEntry {
    pub contents: String,
    pub relative_path: PathBuf,
}

impl FileEntry {
    pub fn get_stem_relative_to(&self, base: &PathBuf) -> String {
        self.relative_path
            .strip_prefix(base)
            .unwrap_or(&self.relative_path)
            .with_extension("")
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
