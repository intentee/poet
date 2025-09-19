use serde::Deserialize;
use serde::Serialize;

use crate::jsonrpc::notification::Notification;
use crate::jsonrpc::request::Request;
use crate::jsonrpc::response::Response;

#[derive(Deserialize, Serialize)]
pub enum Message {
    Notification(Notification),
    Request(Request),
    Response(Response),
}
