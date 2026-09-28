use std::fs::metadata;
use std::path::PathBuf;

use crate::poet_error::PoetError;

pub fn validate_is_directory(path_string: &str) -> Result<PathBuf, PoetError> {
    let path = PathBuf::from(path_string);

    match metadata(&path) {
        Ok(path_metadata) if path_metadata.is_dir() => Ok(path),
        Ok(_) => Err(PoetError::NotADirectory { path }),
        Err(source) => Err(PoetError::InspectPath { path, source }),
    }
}
