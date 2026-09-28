use std::io::Read as _;
use std::io::Write as _;
use std::net::SocketAddr;
use std::net::TcpStream;

use httparse::EMPTY_HEADER;
use httparse::Response;
use httparse::Status;

use crate::http_response::HttpResponse;
use crate::poet_process_tests_error::PoetProcessTestsError;

const MAXIMUM_RESPONSE_HEADERS: usize = 32;

pub fn http_get(address: SocketAddr, path: &str) -> Result<HttpResponse, PoetProcessTestsError> {
    let mut stream = TcpStream::connect(address)?;
    let mut response_bytes = vec![];

    write!(
        stream,
        "GET {path} HTTP/1.1\r\nHost: {address}\r\nConnection: close\r\n\r\n"
    )?;
    stream.read_to_end(&mut response_bytes)?;

    let mut headers = [EMPTY_HEADER; MAXIMUM_RESPONSE_HEADERS];
    let mut response = Response::new(&mut headers);
    let Status::Complete(body_offset) = response.parse(&response_bytes)? else {
        return Err(PoetProcessTestsError::IncompleteHttpResponse);
    };

    Ok(HttpResponse {
        status: response
            .code
            .ok_or(PoetProcessTestsError::MissingHttpStatus)?,
        body: String::from_utf8(response_bytes[body_offset..].to_vec())?,
    })
}
