use std::path::Path;

use poet_filesystem::filesystem::Filesystem as _;
use poet_filesystem::read_file_contents_result::ReadFileContentsResult;
use poet_filesystem_tests::poet_filesystem_tests_error::PoetFilesystemTestsError;
use poet_filesystem_tests::temporary_storage::TemporaryStorage;

#[tokio::test]
async fn storage_reports_missing_file_as_not_found() -> Result<(), PoetFilesystemTestsError> {
    let TemporaryStorage { storage, .. } = TemporaryStorage::create()?;

    assert!(matches!(
        storage.read_file_contents(Path::new("missing.md")).await?,
        ReadFileContentsResult::NotFound
    ));

    Ok(())
}
