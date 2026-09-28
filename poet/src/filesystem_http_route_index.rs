use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;

use poet_content::generated_page_file_name::GENERATED_PAGE_FILE_NAME;
use poet_content::sitemap_file_path::SITEMAP_FILE_PATH;
use poet_filesystem::file_entry::FileEntry;
use poet_filesystem::memory::Memory;

use crate::poet_error::PoetError;

fn routes_of(relative_path: &Path) -> Result<Vec<String>, PoetError> {
    if relative_path == Path::new(SITEMAP_FILE_PATH) {
        Ok(vec![SITEMAP_FILE_PATH.to_owned()])
    } else if relative_path.file_name() == Some(OsStr::new(GENERATED_PAGE_FILE_NAME)) {
        Ok(vec![
            relative_path.display().to_string(),
            relative_path.with_file_name("").display().to_string(),
        ])
    } else {
        Err(PoetError::UnexpectedGeneratedFile {
            relative_path: relative_path.to_path_buf(),
        })
    }
}

pub struct FilesystemHttpRouteIndex {
    routes: HashMap<String, FileEntry>,
}

impl FilesystemHttpRouteIndex {
    pub fn from_memory(memory_filesystem: &Memory) -> Result<Self, PoetError> {
        let mut routes = HashMap::new();

        for file_entry in memory_filesystem.file_entries() {
            for route in routes_of(&file_entry.relative_path)? {
                routes.insert(route, file_entry.clone());
            }
        }

        Ok(Self { routes })
    }

    #[must_use]
    pub fn file_entry_for_route(&self, route: &str) -> Option<&FileEntry> {
        self.routes.get(route)
    }
}
