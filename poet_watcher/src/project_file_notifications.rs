use std::sync::Arc;

use tokio::sync::Notify;

use crate::project_file_kind::ProjectFileKind;

#[derive(Clone, Default)]
pub struct ProjectFileNotifications {
    pub on_author_file_changed: Arc<Notify>,
    pub on_content_file_changed: Arc<Notify>,
    pub on_esbuild_metafile_changed: Arc<Notify>,
    pub on_prompt_file_changed: Arc<Notify>,
    pub on_shortcode_file_changed: Arc<Notify>,
}

impl ProjectFileNotifications {
    #[must_use]
    pub const fn notifier_of(&self, project_file_kind: ProjectFileKind) -> &Arc<Notify> {
        match project_file_kind {
            ProjectFileKind::Author => &self.on_author_file_changed,
            ProjectFileKind::Content => &self.on_content_file_changed,
            ProjectFileKind::EsbuildMetafile => &self.on_esbuild_metafile_changed,
            ProjectFileKind::Prompt => &self.on_prompt_file_changed,
            ProjectFileKind::Shortcode => &self.on_shortcode_file_changed,
        }
    }
}
