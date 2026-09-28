use std::path::Path;
use std::path::PathBuf;

use poet_filesystem::file_entry::FileEntry;
use poet_filesystem::filesystem::Filesystem as _;
use poet_filesystem::source_directory::SourceDirectory;
use poet_filesystem_tests::poet_filesystem_tests_error::PoetFilesystemTestsError;
use poet_filesystem_tests::temporary_storage::TemporaryStorage;

#[tokio::test]
async fn storage_reads_source_files_recursively_by_extension()
-> Result<(), PoetFilesystemTestsError> {
    let TemporaryStorage { storage, .. } = TemporaryStorage::create()?;

    storage
        .set_file_contents(Path::new("content/guide.md"), "guide")
        .await?;
    storage
        .set_file_contents(Path::new("content/docs/intro.md"), "intro")
        .await?;
    storage
        .set_file_contents(Path::new("content/data.toml"), "data")
        .await?;
    storage
        .set_file_contents(Path::new("authors/alice.md"), "alice")
        .await?;

    let mut file_entries = storage
        .read_source_files(&SourceDirectory {
            file_extension: "md",
            name: "content",
        })
        .await?;

    file_entries.sort_by(|first_file_entry, second_file_entry| {
        first_file_entry
            .relative_path
            .cmp(&second_file_entry.relative_path)
    });

    assert_eq!(
        file_entries
            .iter()
            .map(|FileEntry { relative_path, .. }| relative_path.clone())
            .collect::<Vec<PathBuf>>(),
        vec![
            PathBuf::from("content/docs/intro.md"),
            PathBuf::from("content/guide.md"),
        ]
    );
    assert_eq!(
        file_entries
            .iter()
            .map(|FileEntry { contents, .. }| contents.as_str())
            .collect::<Vec<&str>>(),
        vec!["intro", "guide"]
    );

    Ok(())
}
