#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JsonRpcErrorCode {
    InternalError = -32603,
    InvalidParams = -32602,
    InvalidRequest = -32600,
    ParseError = -32700,
    ResourceNotFound = -32002,
}
