use std::fs::create_dir_all;
use std::path::Path;
use std::path::PathBuf;

use poet_filesystem::source_directory::SourceDirectory;

use crate::project_file_kind::ProjectFileKind;
use crate::watcher_error::WatcherError;

pub struct WatchedSourceDirectory {
    pub directory: PathBuf,
    pub file_extension: &'static str,
    pub project_file_kind: ProjectFileKind,
}

impl WatchedSourceDirectory {
    pub fn create_in(
        project_directory: &Path,
        SourceDirectory {
            file_extension,
            name,
        }: &SourceDirectory,
        project_file_kind: ProjectFileKind,
    ) -> Result<Self, WatcherError> {
        let directory = project_directory.join(name);

        match create_dir_all(&directory) {
            Ok(()) => Ok(Self {
                directory,
                file_extension,
                project_file_kind,
            }),
            Err(source) => Err(WatcherError::CreateWatchedDirectory {
                path: directory,
                source,
            }),
        }
    }

    #[must_use]
    pub fn owns(&self, path: &Path) -> bool {
        path.starts_with(&self.directory)
            && path
                .extension()
                .is_none_or(|extension| extension == self.file_extension)
    }
}
