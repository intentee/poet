use std::path::Path;

use poet_filesystem::filesystem::Filesystem as _;
use poet_filesystem::memory::Memory;
use poet_filesystem_tests::poet_filesystem_tests_error::PoetFilesystemTestsError;
use poet_filesystem_tests::temporary_storage::TemporaryStorage;

#[tokio::test]
async fn copies_file_between_filesystems() -> Result<(), PoetFilesystemTestsError> {
    let TemporaryStorage { storage, .. } = TemporaryStorage::create()?;
    let memory = Memory::default();

    storage
        .set_file_contents(Path::new("esbuild-meta.json"), "{}")
        .await?;
    memory
        .copy_file_from(&storage, Path::new("esbuild-meta.json"))
        .await?;

    assert_eq!(
        memory
            .read_file_contents_string(Path::new("esbuild-meta.json"))
            .await?,
        "{}"
    );

    Ok(())
}
