use std::fs::create_dir_all;
use std::path::PathBuf;

use anyhow::Result;

use super::validate_is_directory;

pub fn validate_is_directory_or_create(path_string: &str) -> Result<PathBuf> {
    let path = PathBuf::from(path_string);

    if !path.exists() {
        create_dir_all(&path)?;
    }

    validate_is_directory(path_string)
}
