use std::path::Path;

use poet_filesystem::filesystem::Filesystem as _;
use poet_filesystem::memory::Memory;
use poet_filesystem::read_file_contents_result::ReadFileContentsResult;
use poet_filesystem_tests::poet_filesystem_tests_error::PoetFilesystemTestsError;

#[tokio::test]
async fn memory_reports_missing_file_as_not_found() -> Result<(), PoetFilesystemTestsError> {
    assert!(matches!(
        Memory::default()
            .read_file_contents(Path::new("missing.md"))
            .await?,
        ReadFileContentsResult::NotFound
    ));

    Ok(())
}
