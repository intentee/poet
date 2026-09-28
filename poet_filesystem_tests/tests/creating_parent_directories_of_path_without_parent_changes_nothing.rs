use std::path::Path;

use poet_filesystem::create_parent_directories::create_parent_directories;
use poet_filesystem_tests::poet_filesystem_tests_error::PoetFilesystemTestsError;

#[tokio::test]
async fn creating_parent_directories_of_path_without_parent_changes_nothing()
-> Result<(), PoetFilesystemTestsError> {
    create_parent_directories(Path::new("")).await?;

    Ok(())
}
