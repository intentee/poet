use std::path::Path;

use poet_filesystem::filesystem::Filesystem as _;
use poet_filesystem::filesystem_error::FilesystemError;
use poet_filesystem_tests::poet_filesystem_tests_error::PoetFilesystemTestsError;
use poet_filesystem_tests::temporary_storage::TemporaryStorage;

#[tokio::test]
async fn storage_rejects_writing_over_a_directory() -> Result<(), PoetFilesystemTestsError> {
    let TemporaryStorage { storage, .. } = TemporaryStorage::create()?;

    storage
        .set_file_contents(Path::new("content/guide.md"), "guide")
        .await?;

    assert!(matches!(
        storage
            .set_file_contents(Path::new("content"), "contents")
            .await,
        Err(FilesystemError::WriteFile { .. })
    ));

    Ok(())
}
