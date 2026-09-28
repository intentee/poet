use std::path::Path;

use poet_filesystem::filesystem::Filesystem as _;
use poet_filesystem::filesystem_error::FilesystemError;
use poet_filesystem_tests::poet_filesystem_tests_error::PoetFilesystemTestsError;
use poet_filesystem_tests::temporary_storage::TemporaryStorage;

#[tokio::test]
async fn rejects_reading_directory_as_file() -> Result<(), PoetFilesystemTestsError> {
    let TemporaryStorage { storage, .. } = TemporaryStorage::create()?;

    storage
        .set_file_contents(Path::new("content/guide.md"), "guide")
        .await?;

    assert!(matches!(
        storage.read_file_contents_string(Path::new("content")).await,
        Err(FilesystemError::FileIsDirectory { path }) if path == Path::new("content")
    ));

    Ok(())
}
