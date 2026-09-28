use std::fs::create_dir;
use std::io::ErrorKind;
use std::path::PathBuf;

use crate::cmd::value_parser::validate_is_directory::validate_is_directory;
use crate::poet_error::PoetError;

pub fn validate_is_directory_or_create(path_string: &str) -> Result<PathBuf, PoetError> {
    match create_dir(path_string) {
        Ok(()) => Ok(PathBuf::from(path_string)),
        Err(source) if source.kind() == ErrorKind::AlreadyExists => {
            validate_is_directory(path_string)
        }
        Err(source) => Err(PoetError::CreateOutputDirectory {
            path: PathBuf::from(path_string),
            source,
        }),
    }
}
