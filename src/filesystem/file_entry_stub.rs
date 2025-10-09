use std::path::Component;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct FileEntryStub {
    pub contents: String,
    pub relative_path: PathBuf,
}

impl FileEntryStub {
    pub fn extension(&self) -> Option<String> {
        self.relative_path
            .extension()
            .and_then(|os_str| os_str.to_str())
            .map(|extension_str| extension_str.to_string())
    }

    pub fn top_directory(&self) -> Option<String> {
        self.relative_path
            .components()
            .next()
            .and_then(|component| match component {
                Component::Normal(name) => Some(name.to_os_string().display().to_string()),
                _ => None,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_directory() {
        let file_entry = FileEntryStub {
            contents: String::new(),
            relative_path: PathBuf::from("foo/bar/baz.rhai"),
        };

        assert_eq!(Some("foo".to_string()), file_entry.top_directory());
    }
}
