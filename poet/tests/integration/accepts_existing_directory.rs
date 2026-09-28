use poet::cmd::value_parser::validate_is_directory::validate_is_directory;
use tempfile::tempdir;

use crate::poet_tests_error::PoetTestsError;

#[test]
fn accepts_existing_directory() -> Result<(), PoetTestsError> {
    let directory = tempdir()?;

    assert_eq!(
        validate_is_directory(&directory.path().display().to_string()).map_err(Box::new)?,
        directory.path()
    );

    Ok(())
}
