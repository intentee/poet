use std::path::Path;

use poet_filesystem::filesystem::Filesystem as _;
use poet_filesystem::filesystem_error::FilesystemError;
use poet_filesystem::memory::Memory;
use poet_filesystem::source_directory::SourceDirectory;
use poet_filesystem_tests::poet_filesystem_tests_error::PoetFilesystemTestsError;
use poet_filesystem_tests::temporary_storage::TemporaryStorage;

#[tokio::test]
async fn fails_to_copy_source_files_to_unwritable_filesystem()
-> Result<(), PoetFilesystemTestsError> {
    let TemporaryStorage { storage, .. } = TemporaryStorage::create()?;
    let memory = Memory::default();

    storage
        .set_file_contents(Path::new("content"), "not a directory")
        .await?;
    memory.set_file_contents_sync(Path::new("content/guide.md"), "guide");

    assert!(matches!(
        storage
            .copy_source_files_from(
                &memory,
                &SourceDirectory {
                    file_extension: "md",
                    name: "content",
                }
            )
            .await,
        Err(FilesystemError::CreateParentDirectories { .. })
    ));

    Ok(())
}
