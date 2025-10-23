use std::fs::create_dir_all;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
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
use tokio::sync::Notify;

pub struct WatchProjectHandle {
    pub debouncer: Debouncer<RecommendedWatcher, RecommendedCache>,
    pub on_content_file_changed: Arc<Notify>,
    pub on_esbuild_metafile_changed: Arc<Notify>,
    pub on_prompt_file_changed: Arc<Notify>,
    pub on_shortcode_file_changed: Arc<Notify>,
}

fn is_inside_directory(directory: &Path, file_path: &Path) -> bool {
    if directory == file_path {
        return true;
    }

    file_path
        .canonicalize()
        .and_then(|file| directory.canonicalize().map(|dir| (file, dir)))
        .map(|(file, dir)| file.starts_with(dir))
        .unwrap_or(false)
}

fn is_temp_file(path: &Path) -> bool {
    let path_string = path.to_string_lossy();

    path_string.ends_with("~") || path_string.ends_with(".swp") || path_string.ends_with(".tmp")
}

pub fn watch_project_files(source_directory: PathBuf) -> Result<WatchProjectHandle> {
    let content_directory = source_directory.join("content");
    let esbuild_metafile_path = source_directory.join("esbuild-meta.json");
    let prompts_directory = source_directory.join("prompts");
    let shortcodes_directory = source_directory.join("shortcodes");

    let on_content_file_changed = Arc::new(Notify::new());
    let on_esbuild_metafile_changed = Arc::new(Notify::new());
    let on_prompt_file_changed = Arc::new(Notify::new());
    let on_shortcode_file_changed = Arc::new(Notify::new());

    let content_directory_clone = content_directory.clone();
    let on_shortcode_file_changed_clone = on_shortcode_file_changed.clone();
    let on_content_file_changed_clone = on_content_file_changed.clone();
    let on_esbuild_metafile_changed_clone = on_esbuild_metafile_changed.clone();
    let on_prompt_file_changed_clone = on_prompt_file_changed.clone();
    let prompts_directory_clone = prompts_directory.clone();
    let shortcodes_directory_clone = shortcodes_directory.clone();

    let mut debouncer = new_debouncer(
        Duration::from_millis(100),
        None,
        move |result: DebounceEventResult| match result {
            Ok(events) => {
                for event in &events {
                    match event.kind {
                        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                            for path in &event.paths {
                                if is_temp_file(path) {
                                    info!("Ignoring temporary file change: {}", path.display());

                                    continue;
                                }

                                if is_inside_directory(&shortcodes_directory_clone, path) {
                                    info!("Shortcode file change detected: {:?}", path.display());

                                    on_shortcode_file_changed_clone.notify_waiters();

                                    return;
                                }

                                if is_inside_directory(&prompts_directory_clone, path) {
                                    info!("Prompt file change detected: {:?}", path.display());

                                    on_prompt_file_changed_clone.notify_waiters();

                                    return;
                                }

                                if is_inside_directory(&content_directory_clone, path) {
                                    info!("Content file change detected: {:?}", path.display());

                                    on_content_file_changed_clone.notify_waiters();

                                    return;
                                }

                                if esbuild_metafile_path == *path {
                                    info!("Esbuild metafile change detected: {:?}", path.display());

                                    on_esbuild_metafile_changed_clone.notify_waiters();

                                    return;
                                }

                                info!("Ignoring file change: {:?}", path.display());
                            }

                            return;
                        }
                        _ => {}
                    }
                }
            }
            Err(errors) => errors.iter().for_each(|error| error!("{error:?}")),
        },
    )?;

    create_dir_all(&content_directory)?;
    debouncer.watch(content_directory, RecursiveMode::Recursive)?;

    create_dir_all(&prompts_directory)?;
    debouncer.watch(prompts_directory, RecursiveMode::Recursive)?;

    create_dir_all(&shortcodes_directory)?;
    debouncer.watch(shortcodes_directory, RecursiveMode::Recursive)?;

    debouncer.watch(source_directory.clone(), RecursiveMode::NonRecursive)?;

    Ok(WatchProjectHandle {
        debouncer,
        on_content_file_changed,
        on_esbuild_metafile_changed,
        on_prompt_file_changed,
        on_shortcode_file_changed,
    })
}
