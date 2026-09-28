use std::fs::write;

use poet::cmd::value_parser::validate_is_directory::validate_is_directory;
use poet::poet_error::PoetError;
use tempfile::tempdir;

use crate::poet_tests_error::PoetTestsError;

#[test]
fn rejects_file_as_directory() -> Result<(), PoetTestsError> {
    let directory = tempdir()?;
    let file_path = directory.path().join("file.txt");

    write(&file_path, "contents")?;

    assert!(matches!(
        validate_is_directory(&file_path.display().to_string()),
        Err(PoetError::NotADirectory { path }) if path == file_path
    ));

    Ok(())
}
