use std::path::Path;

use poet_filesystem::filesystem::Filesystem as _;
use poet_filesystem::read_file_contents_result::ReadFileContentsResult;
use poet_filesystem_tests::poet_filesystem_tests_error::PoetFilesystemTestsError;
use poet_filesystem_tests::temporary_storage::TemporaryStorage;

#[tokio::test]
async fn storage_reports_directory_as_directory() -> Result<(), PoetFilesystemTestsError> {
    let TemporaryStorage { storage, .. } = TemporaryStorage::create()?;

    storage
        .set_file_contents(Path::new("content/guide.md"), "guide")
        .await?;

    assert!(matches!(
        storage.read_file_contents(Path::new("content")).await?,
        ReadFileContentsResult::Directory
    ));

    Ok(())
}
