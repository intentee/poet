use std::io;
use std::path::PathBuf;

use freedesktop_entry_parser::ParseError;
use poet_assets::asset_error::AssetError;
use poet_filesystem::filesystem_error::FilesystemError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppDirError {
    #[error("desktop entry attribute '{key}' is defined more than once")]
    AmbiguousDesktopEntryAttribute { key: String },
    #[error("unable to copy assets into the AppDir")]
    CopyAssets(#[source] AssetError),
    #[error("unable to copy project files into the AppDir")]
    CopyProjectFiles(#[source] FilesystemError),
    #[error("desktop entry attribute '{key}' is missing")]
    DesktopEntryAttributeMissing { key: String },
    #[error("desktop entry has no [Desktop Entry] section")]
    DesktopEntrySectionMissing,
    #[error("desktop entry value '{value}' is empty or contains control characters")]
    InvalidDesktopEntryString { value: String },
    #[error("unable to parse desktop entry")]
    ParseDesktopEntry(#[source] ParseError),
    #[error("unable to read desktop entry")]
    ReadDesktopEntry(#[source] FilesystemError),
    #[error("unable to read esbuild metafile")]
    ReadEsbuildMetafile(#[source] AssetError),
    #[error("unable to write AppDir file '{}'", path.display())]
    WriteAppDirFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
}
