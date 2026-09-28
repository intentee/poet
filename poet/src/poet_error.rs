use std::io;
use std::net::SocketAddr;
use std::path::PathBuf;

use poet_app_dir::app_dir_error::AppDirError;
use poet_assets::asset_error::AssetError;
use poet_content::content_error::ContentError;
use poet_filesystem::filesystem_error::FilesystemError;
use poet_mdx::mdx_error::MdxError;
use poet_prompt::prompt_error::PromptError;
use poet_search::search_error::SearchError;
use poet_watcher::watcher_error::WatcherError;
use thiserror::Error;
use tokio::task::JoinError;

#[derive(Debug, Error)]
pub enum PoetError {
    #[error("unable to prepare the AppDir")]
    AppDir(#[from] AppDirError),
    #[error("unable to bind the HTTP server to {address}")]
    BindHttpServer {
        address: SocketAddr,
        #[source]
        source: io::Error,
    },
    #[error("unable to build authors")]
    BuildAuthors(#[source] ContentError),
    #[error("unable to build the project")]
    BuildProject(#[source] ContentError),
    #[error("Server is still starting up, or there are no successful builds yet")]
    BuildProjectResultNotReady,
    #[error("unable to build prompts")]
    BuildPrompts(#[source] PromptError),
    #[error("unable to compile shortcodes")]
    CompileShortcodes(#[source] MdxError),
    #[error("unable to copy assets into the output directory")]
    CopyAssets(#[source] AssetError),
    #[error("unable to create static files directory '{}'", path.display())]
    CreateAssetsDirectory {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("unable to create output directory '{}'", path.display())]
    CreateOutputDirectory {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("unable to index content documents for search")]
    IndexSearch(#[source] SearchError),
    #[error("unable to inspect path '{}'", path.display())]
    InspectPath {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("path '{}' is not a directory", path.display())]
    NotADirectory { path: PathBuf },
    #[error(
        "Prompts are not ready yet. The server is still starting up, or there are no successful prompt builds yet"
    )]
    PromptDocumentControllerCollectionNotReady,
    #[error("unable to read the esbuild metafile")]
    ReadEsbuildMetafile(#[source] AssetError),
    #[error("unable to resolve socket address '{address}'")]
    ResolveSocketAddress {
        address: String,
        #[source]
        source: io::Error,
    },
    #[error("unable to run the HTTP server")]
    RunHttpServer(#[source] io::Error),
    #[error("service task failed")]
    ServiceTask(#[source] JoinError),
    #[error("unable to set the Ctrl-C handler")]
    SetCtrlcHandler(#[source] ctrlc::Error),
    #[error("socket address '{address}' does not resolve to any address")]
    SocketAddressUnresolved { address: String },
    #[error("unexpected generated file '{}'", relative_path.display())]
    UnexpectedGeneratedFile { relative_path: PathBuf },
    #[error("unable to watch project files")]
    WatchProjectFiles(#[from] WatcherError),
    #[error("unable to write generated files into the output directory")]
    WriteGeneratedFiles(#[source] FilesystemError),
}
