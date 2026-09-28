use std::path::Path;

use poet_filesystem::filesystem::Filesystem as _;
use poet_filesystem::filesystem_error::FilesystemError;
use poet_filesystem::memory::Memory;

#[tokio::test]
async fn fails_to_copy_missing_file() {
    assert!(matches!(
        Memory::default()
            .copy_file_from(&Memory::default(), Path::new("missing.json"))
            .await,
        Err(FilesystemError::FileNotFound { path }) if path == Path::new("missing.json")
    ));
}
