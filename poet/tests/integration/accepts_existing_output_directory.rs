use poet::cmd::value_parser::validate_is_directory_or_create::validate_is_directory_or_create;
use tempfile::tempdir;

use crate::poet_tests_error::PoetTestsError;

#[test]
fn accepts_existing_output_directory() -> Result<(), PoetTestsError> {
    let directory = tempdir()?;

    assert_eq!(
        validate_is_directory_or_create(&directory.path().display().to_string())
            .map_err(Box::new)?,
        directory.path()
    );

    Ok(())
}
