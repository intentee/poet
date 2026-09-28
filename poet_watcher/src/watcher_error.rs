use std::io;
use std::path::PathBuf;

use notify_debouncer_full::notify::Error as NotifyError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WatcherError {
    #[error("unable to resolve source directory '{}'", path.display())]
    CanonicalizeSourceDirectory {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("unable to create file watcher")]
    CreateFileWatcher(#[source] NotifyError),
    #[error("unable to create watched directory '{}'", path.display())]
    CreateWatchedDirectory {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("unable to watch directory for changes")]
    WatchDirectory(#[source] NotifyError),
}
