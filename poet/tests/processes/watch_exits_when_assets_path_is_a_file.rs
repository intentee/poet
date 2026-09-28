use crate::fixture_site::FixtureSite;
use crate::free_socket_address::free_socket_address;
use crate::poet_process_tests_error::PoetProcessTestsError;
use crate::run_poet_to_exit::run_poet_to_exit;

#[test]
fn watch_exits_when_assets_path_is_a_file() -> Result<(), PoetProcessTestsError> {
    let fixture_site = FixtureSite::create()?;

    fixture_site.write("assets", "not a directory")?;

    assert!(
        !run_poet_to_exit(&[
            "watch",
            "--addr",
            &free_socket_address()?.to_string(),
            &fixture_site.path_string(),
        ])?
        .success()
    );

    Ok(())
}
