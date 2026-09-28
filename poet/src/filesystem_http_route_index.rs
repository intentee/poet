use std::collections::HashMap;

use poet_content::generated_file::GeneratedFile;
use poet_content::generated_file_kind::GeneratedFileKind;

pub struct FilesystemHttpRouteIndex {
    routes: HashMap<String, GeneratedFile>,
}

impl FilesystemHttpRouteIndex {
    #[must_use]
    pub fn from_generated_files(generated_files: &[GeneratedFile]) -> Self {
        let mut routes = HashMap::new();

        for generated_file in generated_files {
            if generated_file.kind == GeneratedFileKind::Page {
                routes.insert(
                    generated_file
                        .relative_path
                        .with_file_name("")
                        .display()
                        .to_string(),
                    generated_file.clone(),
                );
            }

            routes.insert(
                generated_file.relative_path.display().to_string(),
                generated_file.clone(),
            );
        }

        Self { routes }
    }

    #[must_use]
    pub fn generated_file_for_route(&self, route: &str) -> Option<&GeneratedFile> {
        self.routes.get(route)
    }
}
