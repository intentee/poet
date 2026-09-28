use std::net::Ipv4Addr;
use std::net::TcpListener;

use crate::fixture_site::FixtureSite;
use crate::poet_process_tests_error::PoetProcessTestsError;
use crate::run_poet_to_exit::run_poet_to_exit;

#[test]
fn watch_exits_when_address_is_taken() -> Result<(), PoetProcessTestsError> {
    let fixture_site = FixtureSite::create()?;
    let occupied_listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;

    assert!(
        !run_poet_to_exit(&[
            "watch",
            "--addr",
            &occupied_listener.local_addr()?.to_string(),
            &fixture_site.path_string(),
        ])?
        .success()
    );

    Ok(())
}
