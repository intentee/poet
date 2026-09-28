use std::net::Ipv4Addr;
use std::net::TcpListener;

use poet::poet_error::PoetError;

use crate::fixture_site::FixtureSite;
use crate::poet_tests_error::PoetTestsError;
use crate::run_poet_command::run_poet_command;

#[actix_web::test]
async fn serve_reports_occupied_address() -> Result<(), PoetTestsError> {
    let fixture_site = FixtureSite::create()?;
    let occupied_listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
    let occupied_address = occupied_listener.local_addr()?;

    assert!(matches!(
        run_poet_command(&[
            "serve",
            "--addr",
            &occupied_address.to_string(),
            "--app-name",
            "fixture",
            "--public-path",
            "/",
            &fixture_site.path_string(),
        ])
        .await?,
        Err(PoetError::BindHttpServer { address, .. }) if address == occupied_address
    ));

    Ok(())
}
