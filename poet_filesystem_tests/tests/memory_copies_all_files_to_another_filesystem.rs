use std::path::Path;

use poet_filesystem::filesystem::Filesystem as _;
use poet_filesystem::memory::Memory;
use poet_filesystem_tests::poet_filesystem_tests_error::PoetFilesystemTestsError;
use poet_filesystem_tests::temporary_storage::TemporaryStorage;

#[tokio::test]
async fn memory_copies_all_files_to_another_filesystem() -> Result<(), PoetFilesystemTestsError> {
    let TemporaryStorage { storage, .. } = TemporaryStorage::create()?;
    let memory = Memory::default();

    memory.set_file_contents_sync(Path::new("docs/index.html"), "<p>docs</p>");
    memory.set_file_contents_sync(Path::new("sitemap.xml"), "<urlset/>");
    memory.copy_all_files_to(&storage).await?;

    assert_eq!(
        storage
            .read_file_contents_string(Path::new("docs/index.html"))
            .await?,
        "<p>docs</p>"
    );
    assert_eq!(
        storage
            .read_file_contents_string(Path::new("sitemap.xml"))
            .await?,
        "<urlset/>"
    );

    Ok(())
}
