use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use log::error;
use log::info;
use notify_debouncer_full::DebounceEventResult;
use notify_debouncer_full::Debouncer;
use notify_debouncer_full::RecommendedCache;
use notify_debouncer_full::new_debouncer;
use notify_debouncer_full::notify::RecommendedWatcher;
use notify_debouncer_full::notify::RecursiveMode;
use tokio::sync::Notify;

use crate::cmd::watch::project_file_classifier::ProjectFileClassifier;
use crate::cmd::watch::project_file_kind::ProjectFileKind;

pub struct WatchProjectHandle {
    pub debouncer: Debouncer<RecommendedWatcher, RecommendedCache>,
    pub on_author_file_changed: Arc<Notify>,
    pub on_content_file_changed: Arc<Notify>,
    pub on_esbuild_metafile_changed: Arc<Notify>,
    pub on_prompt_file_changed: Arc<Notify>,
    pub on_shortcode_file_changed: Arc<Notify>,
}

pub fn watch_project_files(source_directory: PathBuf) -> Result<WatchProjectHandle> {
    let canonical_source_directory = source_directory.canonicalize()?;
    let project_file_classifier =
        ProjectFileClassifier::create_watched_directories_in(&canonical_source_directory)?;
    let watched_recursive_directories = [
        project_file_classifier.authors_directory.clone(),
        project_file_classifier.content_directory.clone(),
        project_file_classifier.prompts_directory.clone(),
        project_file_classifier.shortcodes_directory.clone(),
    ];

    let on_author_file_changed = Arc::new(Notify::new());
    let on_content_file_changed = Arc::new(Notify::new());
    let on_esbuild_metafile_changed = Arc::new(Notify::new());
    let on_prompt_file_changed = Arc::new(Notify::new());
    let on_shortcode_file_changed = Arc::new(Notify::new());

    let on_author_file_changed_clone = on_author_file_changed.clone();
    let on_content_file_changed_clone = on_content_file_changed.clone();
    let on_esbuild_metafile_changed_clone = on_esbuild_metafile_changed.clone();
    let on_prompt_file_changed_clone = on_prompt_file_changed.clone();
    let on_shortcode_file_changed_clone = on_shortcode_file_changed.clone();

    let mut debouncer = new_debouncer(
        Duration::from_millis(100),
        None,
        move |result: DebounceEventResult| match result {
            Ok(events) => {
                for project_file_kind in project_file_classifier.changed_kinds(&events) {
                    info!("Project file change detected: {project_file_kind:?}");

                    match project_file_kind {
                        ProjectFileKind::Author => on_author_file_changed_clone.notify_waiters(),
                        ProjectFileKind::Content => on_content_file_changed_clone.notify_waiters(),
                        ProjectFileKind::EsbuildMetafile => {
                            on_esbuild_metafile_changed_clone.notify_waiters()
                        }
                        ProjectFileKind::Prompt => on_prompt_file_changed_clone.notify_waiters(),
                        ProjectFileKind::Shortcode => {
                            on_shortcode_file_changed_clone.notify_waiters()
                        }
                    }
                }
            }
            Err(errors) => errors.iter().for_each(|error| error!("{error:?}")),
        },
    )?;

    for watched_recursive_directory in watched_recursive_directories {
        debouncer.watch(watched_recursive_directory, RecursiveMode::Recursive)?;
    }

    debouncer.watch(canonical_source_directory, RecursiveMode::NonRecursive)?;

    Ok(WatchProjectHandle {
        debouncer,
        on_author_file_changed,
        on_content_file_changed,
        on_esbuild_metafile_changed,
        on_prompt_file_changed,
        on_shortcode_file_changed,
    })
}
