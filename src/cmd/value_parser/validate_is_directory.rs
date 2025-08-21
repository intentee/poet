use std::path::PathBuf;

use anyhow::Result;
use anyhow::anyhow;

pub fn validate_is_directory(path_string: &str) -> Result<PathBuf> {
    let path = PathBuf::from(path_string);

    if !path.exists() {
        return Err(anyhow!("Path does not exist: {path_string}"));
    }

    if !path.is_dir() {
        return Err(anyhow!("Path is not a directory: {path_string}"));
    }

    Ok(path)
}
