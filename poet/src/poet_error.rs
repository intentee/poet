use thiserror::Error;

#[derive(Debug, Error)]
pub enum PoetError {
    #[error("Server is still starting up, or there are no successful builds yet")]
    BuildProjectResultNotReady,
    #[error(
        "Prompts are not ready yet. The server is still starting up, or there are no successful prompt builds yet"
    )]
    PromptDocumentControllerCollectionNotReady,
}
