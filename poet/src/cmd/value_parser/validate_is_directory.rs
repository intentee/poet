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

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn returns_path_for_existing_directory() -> Result<()> {
        let directory = tempdir()?;

        assert_eq!(
            validate_is_directory(&directory.path().display().to_string())?.as_path(),
            directory.path()
        );

        Ok(())
    }

    #[test]
    fn errors_when_path_does_not_exist() -> Result<()> {
        let directory = tempdir()?;
        let missing = directory.path().join("missing");

        assert!(validate_is_directory(&missing.display().to_string()).is_err());

        Ok(())
    }

    #[test]
    fn errors_when_path_is_a_file() -> Result<()> {
        let directory = tempdir()?;
        let file_path = directory.path().join("file.txt");

        fs::write(&file_path, "contents")?;

        assert!(validate_is_directory(&file_path.display().to_string()).is_err());

        Ok(())
    }
}
