use poet_filesystem::filesystem::Filesystem as _;
use poet_filesystem::source_directory::SourceDirectory;
use poet_filesystem_tests::poet_filesystem_tests_error::PoetFilesystemTestsError;
use poet_filesystem_tests::temporary_storage::TemporaryStorage;

#[tokio::test]
async fn storage_reads_no_source_files_from_missing_directory()
-> Result<(), PoetFilesystemTestsError> {
    let TemporaryStorage { storage, .. } = TemporaryStorage::create()?;

    assert!(
        storage
            .read_source_files(&SourceDirectory {
                file_extension: "md",
                name: "content",
            })
            .await?
            .is_empty()
    );

    Ok(())
}
