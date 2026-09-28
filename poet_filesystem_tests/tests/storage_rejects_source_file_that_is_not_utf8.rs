use std::fs::create_dir;
use std::fs::write;

use poet_filesystem::filesystem::Filesystem as _;
use poet_filesystem::filesystem_error::FilesystemError;
use poet_filesystem::source_directory::SourceDirectory;
use poet_filesystem_tests::poet_filesystem_tests_error::PoetFilesystemTestsError;
use poet_filesystem_tests::temporary_storage::TemporaryStorage;

#[tokio::test]
async fn storage_rejects_source_file_that_is_not_utf8() -> Result<(), PoetFilesystemTestsError> {
    let TemporaryStorage { directory, storage } = TemporaryStorage::create()?;

    create_dir(directory.path().join("content"))?;
    write(directory.path().join("content/binary.md"), [0xff, 0xfe])?;

    assert!(matches!(
        storage
            .read_source_files(&SourceDirectory {
                file_extension: "md",
                name: "content",
            })
            .await,
        Err(FilesystemError::ReadFile { .. })
    ));

    Ok(())
}
