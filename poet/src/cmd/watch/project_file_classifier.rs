use std::collections::BTreeSet;
use std::fs::create_dir_all;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use notify_debouncer_full::DebouncedEvent;
use notify_debouncer_full::notify::EventKind;
use poet_assets::esbuild_metafile_path::ESBUILD_METAFILE_PATH;
use poet_mdx::shortcodes_source_directory::SHORTCODES_SOURCE_DIRECTORY;

use crate::authors_source_directory::AUTHORS_SOURCE_DIRECTORY;
use crate::cmd::watch::project_file_kind::ProjectFileKind;
use crate::content_source_directory::CONTENT_SOURCE_DIRECTORY;
use crate::prompts_source_directory::PROMPTS_SOURCE_DIRECTORY;

fn is_temp_file(path: &Path) -> bool {
    let path_string = path.to_string_lossy();

    path_string.ends_with("~") || path_string.ends_with(".swp") || path_string.ends_with(".tmp")
}

fn create_watched_directory(source_directory: &Path, subdirectory: &str) -> Result<PathBuf> {
    let watched_directory = source_directory.join(subdirectory);

    create_dir_all(&watched_directory)?;

    Ok(watched_directory)
}

pub struct ProjectFileClassifier {
    pub authors_directory: PathBuf,
    pub content_directory: PathBuf,
    pub esbuild_metafile_path: PathBuf,
    pub prompts_directory: PathBuf,
    pub shortcodes_directory: PathBuf,
}

impl ProjectFileClassifier {
    pub fn create_watched_directories_in(source_directory: &Path) -> Result<Self> {
        Ok(Self {
            authors_directory: create_watched_directory(
                source_directory,
                AUTHORS_SOURCE_DIRECTORY.name,
            )?,
            content_directory: create_watched_directory(
                source_directory,
                CONTENT_SOURCE_DIRECTORY.name,
            )?,
            esbuild_metafile_path: source_directory.join(ESBUILD_METAFILE_PATH),
            prompts_directory: create_watched_directory(
                source_directory,
                PROMPTS_SOURCE_DIRECTORY.name,
            )?,
            shortcodes_directory: create_watched_directory(
                source_directory,
                SHORTCODES_SOURCE_DIRECTORY.name,
            )?,
        })
    }

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
            .filter(|path| !is_temp_file(path))
            .filter_map(|path| self.classify(path))
            .collect()
    }

    fn classify(&self, path: &Path) -> Option<ProjectFileKind> {
        if path.starts_with(&self.shortcodes_directory) {
            Some(ProjectFileKind::Shortcode)
        } else if path.starts_with(&self.prompts_directory) {
            Some(ProjectFileKind::Prompt)
        } else if path.starts_with(&self.content_directory) {
            Some(ProjectFileKind::Content)
        } else if path.starts_with(&self.authors_directory) {
            Some(ProjectFileKind::Author)
        } else if path == self.esbuild_metafile_path {
            Some(ProjectFileKind::EsbuildMetafile)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::fs::create_dir_all;
    use std::fs::write;
    use std::path::Path;
    use std::time::Instant;

    use anyhow::Result;
    use notify_debouncer_full::DebouncedEvent;
    use notify_debouncer_full::notify::Event;
    use notify_debouncer_full::notify::EventKind;
    use notify_debouncer_full::notify::event::ModifyKind;
    use notify_debouncer_full::notify::event::RemoveKind;
    use poet_assets::esbuild_metafile_path::ESBUILD_METAFILE_PATH;
    use poet_mdx::shortcodes_source_directory::SHORTCODES_SOURCE_DIRECTORY;
    use tempfile::TempDir;
    use tempfile::tempdir;

    use super::ProjectFileClassifier;
    use crate::authors_source_directory::AUTHORS_SOURCE_DIRECTORY;
    use crate::cmd::watch::project_file_kind::ProjectFileKind;
    use crate::content_source_directory::CONTENT_SOURCE_DIRECTORY;
    use crate::prompts_source_directory::PROMPTS_SOURCE_DIRECTORY;

    fn project_directory() -> Result<TempDir> {
        let project_directory = tempdir()?;

        for subdirectory in [
            AUTHORS_SOURCE_DIRECTORY.name,
            CONTENT_SOURCE_DIRECTORY.name,
            PROMPTS_SOURCE_DIRECTORY.name,
            SHORTCODES_SOURCE_DIRECTORY.name,
        ] {
            create_dir_all(project_directory.path().join(subdirectory))?;
        }

        Ok(project_directory)
    }

    fn classifier(source_directory: &Path) -> ProjectFileClassifier {
        ProjectFileClassifier {
            authors_directory: source_directory.join(AUTHORS_SOURCE_DIRECTORY.name),
            content_directory: source_directory.join(CONTENT_SOURCE_DIRECTORY.name),
            esbuild_metafile_path: source_directory.join(ESBUILD_METAFILE_PATH),
            prompts_directory: source_directory.join(PROMPTS_SOURCE_DIRECTORY.name),
            shortcodes_directory: source_directory.join(SHORTCODES_SOURCE_DIRECTORY.name),
        }
    }

    fn event(event_kind: EventKind, path: &Path) -> DebouncedEvent {
        DebouncedEvent::new(
            Event::new(event_kind).add_path(path.to_path_buf()),
            Instant::now(),
        )
    }

    #[test]
    fn reports_every_kind_changed_in_one_batch() -> Result<()> {
        let project_directory = project_directory()?;
        let shortcode_path = project_directory.path().join("shortcodes/Layout.rhai");
        let content_path = project_directory.path().join("content/guide.md");

        write(&shortcode_path, "shortcode")?;
        write(&content_path, "content")?;

        assert_eq!(
            classifier(project_directory.path()).changed_kinds(&[
                event(EventKind::Modify(ModifyKind::Any), &shortcode_path),
                event(EventKind::Modify(ModifyKind::Any), &content_path),
            ]),
            BTreeSet::from([ProjectFileKind::Content, ProjectFileKind::Shortcode])
        );

        Ok(())
    }

    #[test]
    fn reports_removed_content_file() -> Result<()> {
        let project_directory = project_directory()?;
        let removed_path = project_directory.path().join("content/removed.md");

        assert_eq!(
            classifier(project_directory.path())
                .changed_kinds(&[event(EventKind::Remove(RemoveKind::File), &removed_path)]),
            BTreeSet::from([ProjectFileKind::Content])
        );

        Ok(())
    }
}
