pub mod error;
pub mod success;

use serde::Deserialize;
use serde::Serialize;

use crate::jsonrpc::response::error::Error;
use crate::jsonrpc::response::success::Success;

#[derive(Debug, Deserialize, Serialize)]
pub enum Response {
    Error(Error),
    Success(Success),
}
