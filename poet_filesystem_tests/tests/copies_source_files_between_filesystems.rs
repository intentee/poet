use std::path::Path;

use poet_filesystem::filesystem::Filesystem as _;
use poet_filesystem::memory::Memory;
use poet_filesystem::source_directory::SourceDirectory;
use poet_filesystem_tests::poet_filesystem_tests_error::PoetFilesystemTestsError;
use poet_filesystem_tests::temporary_storage::TemporaryStorage;

#[tokio::test]
async fn copies_source_files_between_filesystems() -> Result<(), PoetFilesystemTestsError> {
    let TemporaryStorage { storage, .. } = TemporaryStorage::create()?;
    let memory = Memory::default();

    memory.set_file_contents_sync(Path::new("content/docs/intro.md"), "intro");
    storage
        .copy_source_files_from(
            &memory,
            &SourceDirectory {
                file_extension: "md",
                name: "content",
            },
        )
        .await?;

    assert_eq!(
        storage
            .read_file_contents_string(Path::new("content/docs/intro.md"))
            .await?,
        "intro"
    );

    Ok(())
}
