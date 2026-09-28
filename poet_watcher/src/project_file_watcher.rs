use notify_debouncer_full::Debouncer;
use notify_debouncer_full::RecommendedCache;
use notify_debouncer_full::notify::RecommendedWatcher;

use crate::project_file_notifications::ProjectFileNotifications;

pub struct ProjectFileWatcher {
    pub debouncer: Debouncer<RecommendedWatcher, RecommendedCache>,
    pub notifications: ProjectFileNotifications,
}
