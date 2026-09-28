use log::error;
use log::info;
use notify_debouncer_full::DebounceEventHandler;
use notify_debouncer_full::DebounceEventResult;

use crate::project_file_classifier::ProjectFileClassifier;
use crate::project_file_notifications::ProjectFileNotifications;

pub struct ProjectFileChangeNotifier {
    pub notifications: ProjectFileNotifications,
    pub project_file_classifier: ProjectFileClassifier,
}

impl DebounceEventHandler for ProjectFileChangeNotifier {
    fn handle_event(&mut self, event: DebounceEventResult) {
        match event {
            Ok(events) => {
                for project_file_kind in self.project_file_classifier.changed_kinds(&events) {
                    info!("Project file change detected: {project_file_kind:?}");

                    self.notifications
                        .notifier_of(project_file_kind)
                        .notify_one();
                }
            }
            Err(errors) => {
                for watch_error in errors {
                    error!("Unable to watch project files: {watch_error}");
                }
            }
        }
    }
}
