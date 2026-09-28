use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use notify_debouncer_full::new_debouncer;
use notify_debouncer_full::notify::RecursiveMode;

use crate::project_file_change_notifier::ProjectFileChangeNotifier;
use crate::project_file_classifier::ProjectFileClassifier;
use crate::project_file_notifications::ProjectFileNotifications;
use crate::project_file_watcher::ProjectFileWatcher;
use crate::watcher_error::WatcherError;

const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(100);

pub fn watch_project_files(project_directory: &Path) -> Result<ProjectFileWatcher, WatcherError> {
    let canonical_project_directory = project_directory.canonicalize().map_err(|source| {
        WatcherError::CanonicalizeSourceDirectory {
            path: project_directory.to_path_buf(),
            source,
        }
    })?;
    let project_file_classifier =
        ProjectFileClassifier::create_watched_directories_in(&canonical_project_directory)?;
    let watched_recursive_directories: Vec<PathBuf> = project_file_classifier
        .watched_source_directories
        .iter()
        .map(|watched_source_directory| watched_source_directory.directory.clone())
        .collect();
    let notifications = ProjectFileNotifications::default();

    new_debouncer(
        DEBOUNCE_TIMEOUT,
        None,
        ProjectFileChangeNotifier {
            notifications: notifications.clone(),
            project_file_classifier,
        },
    )
    .map_err(WatcherError::CreateFileWatcher)
    .and_then(|mut debouncer| {
        watched_recursive_directories
            .iter()
            .try_for_each(|watched_recursive_directory| {
                debouncer.watch(watched_recursive_directory, RecursiveMode::Recursive)
            })
            .and_then(|()| {
                debouncer.watch(&canonical_project_directory, RecursiveMode::NonRecursive)
            })
            .map_err(WatcherError::WatchDirectory)
            .map(|()| ProjectFileWatcher {
                debouncer,
                notifications,
            })
    })
}
