use std::fs::create_dir;

use poet_filesystem::filesystem::Filesystem as _;
use poet_filesystem::filesystem_error::FilesystemError;
use poet_filesystem::memory::Memory;
use poet_filesystem::source_directory::SourceDirectory;
use poet_filesystem_tests::deny_access::deny_access;
use poet_filesystem_tests::grant_owner_access::grant_owner_access;
use poet_filesystem_tests::poet_filesystem_tests_error::PoetFilesystemTestsError;
use poet_filesystem_tests::temporary_storage::TemporaryStorage;

#[tokio::test]
async fn fails_to_copy_source_files_from_unreadable_filesystem()
-> Result<(), PoetFilesystemTestsError> {
    let TemporaryStorage { directory, storage } = TemporaryStorage::create()?;
    let content_directory = directory.path().join("content");

    create_dir(&content_directory)?;
    deny_access(&content_directory)?;

    let copy_result = Memory::default()
        .copy_source_files_from(
            &storage,
            &SourceDirectory {
                file_extension: "md",
                name: "content",
            },
        )
        .await;

    grant_owner_access(&content_directory)?;

    assert!(matches!(
        copy_result,
        Err(FilesystemError::ReadDirectory { .. })
    ));

    Ok(())
}
