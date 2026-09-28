use serde_json::from_slice;
use std::pin::Pin;

use actix_web::body::BoxBody;
use actix_web::body::MessageBody as _;
use actix_web::web::Bytes;
use futures_util::future::poll_fn;
use serde_json::Value;

use crate::poet_mcp_tests_error::PoetMcpTestsError;

const COMMENT_PREFIX: &[u8] = b":";
const DATA_PREFIX: &[u8] = b"data: ";

pub struct EventStream {
    pub body: BoxBody,
}

impl EventStream {
    pub async fn ended(mut self) -> Result<(), PoetMcpTestsError> {
        while let Some(chunk) = self.next_chunk().await? {
            if !chunk.starts_with(COMMENT_PREFIX) {
                return Err(PoetMcpTestsError::UnexpectedEventChunk { chunk });
            }
        }

        Ok(())
    }

    pub async fn next_event(&mut self) -> Result<Value, PoetMcpTestsError> {
        loop {
            let chunk = self
                .next_chunk()
                .await?
                .ok_or(PoetMcpTestsError::EventStreamEnded)?;

            if !chunk.starts_with(COMMENT_PREFIX) {
                let event_data = chunk.strip_prefix(DATA_PREFIX).ok_or_else(|| {
                    PoetMcpTestsError::UnexpectedEventChunk {
                        chunk: chunk.clone(),
                    }
                })?;

                return from_slice(event_data).map_err(PoetMcpTestsError::ParseEvent);
            }
        }
    }

    async fn next_chunk(&mut self) -> Result<Option<Bytes>, PoetMcpTestsError> {
        poll_fn(|context| Pin::new(&mut self.body).poll_next(context))
            .await
            .transpose()
            .map_err(PoetMcpTestsError::ReadEventStream)
    }
}
