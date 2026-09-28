use std::path::Path;
use std::path::PathBuf;

use poet_filesystem::file_entry::FileEntry;
use poet_filesystem::filesystem::Filesystem as _;
use poet_filesystem::memory::Memory;
use poet_filesystem::source_directory::SourceDirectory;
use poet_filesystem_tests::poet_filesystem_tests_error::PoetFilesystemTestsError;

#[tokio::test]
async fn memory_reads_source_files_by_directory_and_extension()
-> Result<(), PoetFilesystemTestsError> {
    let memory = Memory::default();

    memory.set_file_contents_sync(Path::new("content/guide.md"), "guide");
    memory.set_file_contents_sync(Path::new("content/data.toml"), "data");
    memory.set_file_contents_sync(Path::new("authors/alice.md"), "alice");

    assert_eq!(
        memory
            .read_source_files(&SourceDirectory {
                file_extension: "md",
                name: "content",
            })
            .await?
            .into_iter()
            .map(|FileEntry { relative_path, .. }| relative_path)
            .collect::<Vec<PathBuf>>(),
        vec![PathBuf::from("content/guide.md")]
    );

    Ok(())
}
