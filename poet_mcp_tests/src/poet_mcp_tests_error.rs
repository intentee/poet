use std::error::Error as StdError;

use actix_web::http::header::InvalidHeaderValue;
use actix_web::http::header::ToStrError;
use actix_web::web::Bytes;
use poet_mcp::mcp_error::McpError;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PoetMcpTestsError {
    #[error("event stream ended before the expected event arrived")]
    EventStreamEnded,
    #[error("invalid header value")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    #[error("session header is not valid text")]
    InvalidSessionHeader(#[source] ToStrError),
    #[error("unable to convert test value to or from JSON")]
    JsonConversion(#[from] serde_json::Error),
    #[error("MCP server failed")]
    Mcp(#[from] McpError),
    #[error("initialize response does not contain the session header")]
    MissingSessionHeader,
    #[error("unable to parse event stream data")]
    ParseEvent(#[source] serde_json::Error),
    #[error("unable to parse response body")]
    ParseResponseBody(#[source] serde_json::Error),
    #[error("unable to read event stream")]
    ReadEventStream(#[source] Box<dyn StdError>),
    #[error("unable to read response body")]
    ReadResponseBody(#[source] Box<dyn StdError>),
    #[error("event stream sent an unexpected event: {event}")]
    UnexpectedEvent { event: Value },
    #[error("event stream sent a chunk that is neither data nor a comment: {chunk:?}")]
    UnexpectedEventChunk { chunk: Bytes },
}
