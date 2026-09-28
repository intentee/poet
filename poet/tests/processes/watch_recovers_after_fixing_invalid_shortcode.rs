use std::fs::remove_file;

use crate::fixture_site::FixtureSite;
use crate::free_socket_address::free_socket_address;
use crate::http_get::http_get;
use crate::http_response::HttpResponse;
use crate::poet_process_tests_error::PoetProcessTestsError;
use crate::poet_server_run::PoetServerRun;
use crate::run_poet_server::run_poet_server;
use crate::wait_for_page::wait_for_page;

#[test]
fn watch_recovers_after_fixing_invalid_shortcode() -> Result<(), PoetProcessTestsError> {
    let fixture_site = FixtureSite::create()?;
    let address = free_socket_address()?;

    fixture_site.write("shortcodes/Broken.rhai", "fn template(")?;

    let PoetServerRun {
        exit_status,
        outcome,
    } = run_poet_server(
        &[
            "watch",
            "--addr",
            &address.to_string(),
            &fixture_site.path_string(),
        ],
        address,
        || -> Result<HttpResponse, PoetProcessTestsError> {
            let unavailable_response = http_get(address, "/")?;

            remove_file(fixture_site.path().join("shortcodes/Broken.rhai"))?;
            wait_for_page(address, "/")?;

            Ok(unavailable_response)
        },
    )?;

    assert!(exit_status.success());
    assert!(matches!(outcome, Ok(HttpResponse { status: 503, .. })));

    Ok(())
}
