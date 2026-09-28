use poet::cmd::value_parser::validate_is_directory::validate_is_directory;
use poet::poet_error::PoetError;
use tempfile::tempdir;

use crate::poet_tests_error::PoetTestsError;

#[test]
fn rejects_missing_directory() -> Result<(), PoetTestsError> {
    let directory = tempdir()?;
    let missing_path = directory.path().join("missing");

    assert!(matches!(
        validate_is_directory(&missing_path.display().to_string()),
        Err(PoetError::InspectPath { path, .. }) if path == missing_path
    ));

    Ok(())
}
