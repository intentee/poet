use poet::cmd::value_parser::validate_is_directory_or_create::validate_is_directory_or_create;
use tempfile::tempdir;

use crate::poet_tests_error::PoetTestsError;

#[test]
fn creates_missing_output_directory() -> Result<(), PoetTestsError> {
    let directory = tempdir()?;
    let created_path = directory.path().join("created");

    assert!(
        validate_is_directory_or_create(&created_path.display().to_string())
            .map_err(Box::new)?
            .is_dir()
    );

    Ok(())
}
