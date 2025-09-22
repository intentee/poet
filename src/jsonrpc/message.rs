use serde::Deserialize;
use serde::Serialize;

use crate::jsonrpc::notification::Notification;
use crate::jsonrpc::request::Request;
use crate::jsonrpc::response::Response;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Message {
    Request(Request),
    Response(Response),
    Notification(Notification),
}
