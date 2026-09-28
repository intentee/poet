use actix_web::HttpRequest;
use actix_web::http::header::Accept;
use actix_web::http::header::Header as _;
use mime::Mime;

use crate::mcp_error::McpError;

fn accepts(accepted_mimes: &[Mime], required_mime: &Mime) -> bool {
    accepted_mimes.iter().any(|accepted_mime| {
        accepted_mime.type_() == mime::STAR
            || (accepted_mime.type_() == required_mime.type_()
                && (accepted_mime.subtype() == mime::STAR
                    || accepted_mime.subtype() == required_mime.subtype()))
    })
}

pub fn accepts_all(http_request: &HttpRequest, required_mimes: &[Mime]) -> Result<(), McpError> {
    let accepted_mimes = Accept::parse(http_request)
        .map_err(|source| McpError::InvalidAcceptHeader { source })?
        .ranked();

    if required_mimes
        .iter()
        .all(|required_mime| accepts(&accepted_mimes, required_mime))
    {
        Ok(())
    } else {
        Err(McpError::NotAcceptable)
    }
}
