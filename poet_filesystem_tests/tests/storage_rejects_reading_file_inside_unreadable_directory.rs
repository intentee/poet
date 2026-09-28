use std::path::Path;

use poet_filesystem::filesystem::Filesystem as _;
use poet_filesystem::filesystem_error::FilesystemError;
use poet_filesystem_tests::deny_access::deny_access;
use poet_filesystem_tests::grant_owner_access::grant_owner_access;
use poet_filesystem_tests::poet_filesystem_tests_error::PoetFilesystemTestsError;
use poet_filesystem_tests::temporary_storage::TemporaryStorage;

#[tokio::test]
async fn storage_rejects_reading_file_inside_unreadable_directory()
-> Result<(), PoetFilesystemTestsError> {
    let TemporaryStorage { directory, storage } = TemporaryStorage::create()?;
    let content_directory = directory.path().join("content");

    storage
        .set_file_contents(Path::new("content/guide.md"), "guide")
        .await?;
    deny_access(&content_directory)?;

    let read_result = storage
        .read_file_contents_string(Path::new("content/guide.md"))
        .await;

    grant_owner_access(&content_directory)?;

    assert!(matches!(
        read_result,
        Err(FilesystemError::ReadMetadata { .. })
    ));

    Ok(())
}
