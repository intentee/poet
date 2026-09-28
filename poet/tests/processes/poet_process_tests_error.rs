use std::io;
use std::num::TryFromIntError;
use std::process::ExitStatus;
use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PoetProcessTestsError {
    #[error("HTTP response is incomplete")]
    IncompleteHttpResponse,
    #[error("HTTP response has no status code")]
    MissingHttpStatus,
    #[error("HTTP response body is not UTF-8")]
    NonUtf8HttpBody(#[from] FromUtf8Error),
    #[error("unable to parse HTTP response")]
    ParseHttpResponse(#[from] httparse::Error),
    #[error("unable to prepare test fixture on disk")]
    PrepareFixture(#[from] io::Error),
    #[error("poet process exited with {exit_status} before listening")]
    ProcessExitedBeforeListening { exit_status: ExitStatus },
    #[error("process id does not fit a signal target")]
    ProcessId(#[from] TryFromIntError),
    #[error("unable to signal the poet process")]
    SignalProcess(#[from] nix::errno::Errno),
    #[error("live reload socket failed")]
    WebSocket(#[from] Box<tungstenite::Error>),
}
