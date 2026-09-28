use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use notify_debouncer_full::DebouncedEvent;
use notify_debouncer_full::notify::EventKind;
use poet_assets::esbuild_metafile_path::ESBUILD_METAFILE_PATH;
use poet_content::authors_source_directory::AUTHORS_SOURCE_DIRECTORY;
use poet_content::content_source_directory::CONTENT_SOURCE_DIRECTORY;
use poet_mdx::shortcodes_source_directory::SHORTCODES_SOURCE_DIRECTORY;
use poet_prompt::prompts_source_directory::PROMPTS_SOURCE_DIRECTORY;

use crate::project_file_kind::ProjectFileKind;
use crate::watched_source_directory::WatchedSourceDirectory;
use crate::watcher_error::WatcherError;

pub struct ProjectFileClassifier {
    pub esbuild_metafile_path: PathBuf,
    pub watched_source_directories: Vec<WatchedSourceDirectory>,
}

impl ProjectFileClassifier {
    pub fn create_watched_directories_in(project_directory: &Path) -> Result<Self, WatcherError> {
        [
            WatchedSourceDirectory::create_in(
                project_directory,
                &AUTHORS_SOURCE_DIRECTORY,
                ProjectFileKind::Author,
            ),
            WatchedSourceDirectory::create_in(
                project_directory,
                &CONTENT_SOURCE_DIRECTORY,
                ProjectFileKind::Content,
            ),
            WatchedSourceDirectory::create_in(
                project_directory,
                &PROMPTS_SOURCE_DIRECTORY,
                ProjectFileKind::Prompt,
            ),
            WatchedSourceDirectory::create_in(
                project_directory,
                &SHORTCODES_SOURCE_DIRECTORY,
                ProjectFileKind::Shortcode,
            ),
        ]
        .into_iter()
        .collect::<Result<Vec<WatchedSourceDirectory>, WatcherError>>()
        .map(|watched_source_directories| Self {
            esbuild_metafile_path: project_directory.join(ESBUILD_METAFILE_PATH),
            watched_source_directories,
        })
    }

    #[must_use]
    pub fn changed_kinds(&self, events: &[DebouncedEvent]) -> BTreeSet<ProjectFileKind> {
        events
            .iter()
            .filter(|event| {
                matches!(
                    event.kind,
                    EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
                )
            })
            .flat_map(|event| event.paths.iter())
            .filter_map(|path| self.classify(path))
            .collect()
    }

    fn classify(&self, path: &Path) -> Option<ProjectFileKind> {
        if path == self.esbuild_metafile_path {
            Some(ProjectFileKind::EsbuildMetafile)
        } else {
            self.watched_source_directories
                .iter()
                .find(|watched_source_directory| watched_source_directory.owns(path))
                .map(|watched_source_directory| watched_source_directory.project_file_kind)
        }
    }
}
