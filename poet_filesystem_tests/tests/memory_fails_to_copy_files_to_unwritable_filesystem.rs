use std::path::Path;

use poet_filesystem::filesystem::Filesystem as _;
use poet_filesystem::filesystem_error::FilesystemError;
use poet_filesystem::memory::Memory;
use poet_filesystem_tests::poet_filesystem_tests_error::PoetFilesystemTestsError;
use poet_filesystem_tests::temporary_storage::TemporaryStorage;

#[tokio::test]
async fn memory_fails_to_copy_files_to_unwritable_filesystem()
-> Result<(), PoetFilesystemTestsError> {
    let TemporaryStorage { storage, .. } = TemporaryStorage::create()?;
    let memory = Memory::default();

    storage
        .set_file_contents(Path::new("docs"), "not a directory")
        .await?;
    memory.set_file_contents_sync(Path::new("docs/index.html"), "<p>docs</p>");

    assert!(matches!(
        memory.copy_all_files_to(&storage).await,
        Err(FilesystemError::CreateParentDirectories { .. })
    ));

    Ok(())
}
