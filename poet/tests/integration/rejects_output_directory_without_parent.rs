use poet::cmd::value_parser::validate_is_directory_or_create::validate_is_directory_or_create;
use poet::poet_error::PoetError;
use tempfile::tempdir;

use crate::poet_tests_error::PoetTestsError;

#[test]
fn rejects_output_directory_without_parent() -> Result<(), PoetTestsError> {
    let directory = tempdir()?;
    let nested_path = directory.path().join("missing/nested");

    assert!(matches!(
        validate_is_directory_or_create(&nested_path.display().to_string()),
        Err(PoetError::CreateOutputDirectory { path, .. }) if path == nested_path
    ));

    Ok(())
}
