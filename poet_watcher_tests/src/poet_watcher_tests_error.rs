use std::io;

use poet_watcher::watcher_error::WatcherError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PoetWatcherTestsError {
    #[error("unable to prepare test fixture on disk")]
    PrepareFixture(#[from] io::Error),
    #[error("watcher operation failed")]
    Watcher(#[from] WatcherError),
}
