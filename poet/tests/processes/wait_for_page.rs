use std::net::SocketAddr;

use crate::http_get::http_get;
use crate::http_response::HttpResponse;
use crate::poet_process_tests_error::PoetProcessTestsError;

const HTTP_STATUS_OK: u16 = 200;

pub fn wait_for_page(
    address: SocketAddr,
    path: &str,
) -> Result<HttpResponse, PoetProcessTestsError> {
    loop {
        let http_response = http_get(address, path)?;

        if http_response.status == HTTP_STATUS_OK {
            return Ok(http_response);
        }
    }
}
