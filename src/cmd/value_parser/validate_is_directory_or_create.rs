use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use anyhow::anyhow;

pub fn validate_is_directory_or_create(path_string: &str) -> Result<PathBuf> {
    let path = PathBuf::from(path_string);

    if !path.exists() {
        fs::create_dir(&path)?;
    }

    if !path.is_dir() {
        return Err(anyhow!("Path is not a directory: {path_string}"));
    }

    Ok(path)
}
