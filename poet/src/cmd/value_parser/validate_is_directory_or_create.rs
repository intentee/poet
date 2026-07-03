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

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn returns_existing_directory() -> Result<()> {
        let directory = tempdir()?;

        assert_eq!(
            validate_is_directory_or_create(&directory.path().display().to_string())?.as_path(),
            directory.path()
        );

        Ok(())
    }

    #[test]
    fn creates_directory_when_missing() -> Result<()> {
        let directory = tempdir()?;
        let created = directory.path().join("created");

        assert!(validate_is_directory_or_create(&created.display().to_string())?.is_dir());

        Ok(())
    }

    #[test]
    fn errors_when_path_is_a_file() -> Result<()> {
        let directory = tempdir()?;
        let file_path = directory.path().join("file.txt");

        fs::write(&file_path, "contents")?;

        assert!(validate_is_directory_or_create(&file_path.display().to_string()).is_err());

        Ok(())
    }
}
