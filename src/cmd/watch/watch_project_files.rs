use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;
use log::error;
use log::info;
use notify::EventKind;
use notify::RecommendedWatcher;
use notify_debouncer_full::DebounceEventResult;
use notify_debouncer_full::Debouncer;
use notify_debouncer_full::RecommendedCache;
use notify_debouncer_full::new_debouncer;
use notify_debouncer_full::notify::RecursiveMode;
use tokio::sync::watch;

pub struct WatchProjectHandle {
    pub content_files_changed_rx: watch::Receiver<()>,
    pub debouncer: Debouncer<RecommendedWatcher, RecommendedCache>,
}

pub fn watch_project_files(source_directory: PathBuf) -> Result<WatchProjectHandle> {
    let (content_files_changed_tx, content_files_changed_rx) = watch::channel(());

    content_files_changed_tx
        .send(())
        .expect("Failed to send file change notification");

    let mut debouncer = new_debouncer(
        Duration::from_millis(30),
        None,
        move |result: DebounceEventResult| match result {
            Ok(events) => {
                for event in &events {
                    match event.kind {
                        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                            info!("Source file change detected: {:?}", event.paths);

                            for path in &event.paths {
                                // try to ignore temporary files
                                let path_string = path.to_string_lossy();

                                if path_string.ends_with("~")
                                    || path_string.ends_with(".swp")
                                    || path_string.ends_with(".tmp")
                                {
                                    continue;
                                }

                                content_files_changed_tx
                                    .send(())
                                    .expect("Failed to send file change notification");
                            }

                            break;
                        }
                        _ => {}
                    }
                }
            }
            Err(errors) => errors.iter().for_each(|error| error!("{error:?}")),
        },
    )?;

    debouncer.watch(source_directory.join("content"), RecursiveMode::Recursive)?;
    debouncer.watch(source_directory.clone(), RecursiveMode::NonRecursive)?;
    debouncer.watch(
        source_directory.join("shortcodes"),
        RecursiveMode::Recursive,
    )?;

    Ok(WatchProjectHandle {
        content_files_changed_rx,
        debouncer,
    })
}
