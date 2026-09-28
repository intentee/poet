use std::fs::write;

use poet_content::content_error::ContentError;
use poet_content::load_content_document_sources::load_content_document_sources;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use poet_filesystem::storage::Storage;
use tempfile::tempdir;

#[tokio::test]
async fn rejects_unreadable_content_directory() -> Result<(), PoetContentTestsError> {
    let directory = tempdir()?;
    let regular_file = directory.path().join("regular-file");

    write(&regular_file, "not a directory")?;

    assert!(matches!(
        load_content_document_sources(
            &Storage {
                base_directory: regular_file,
            },
            "/",
        )
        .await,
        Err(ContentError::ReadContentFiles(_))
    ));

    Ok(())
}
