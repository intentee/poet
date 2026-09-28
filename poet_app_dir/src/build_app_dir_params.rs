use std::path::Path;

use crate::app_dir_desktop_entry::AppDirDesktopEntry;

pub struct BuildAppDirParams<'params> {
    pub desktop_entry: AppDirDesktopEntry,
    pub output_directory: &'params Path,
    pub source_directory: &'params Path,
}
