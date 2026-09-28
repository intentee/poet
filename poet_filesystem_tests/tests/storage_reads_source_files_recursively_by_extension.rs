use std::path::Path;
use std::path::PathBuf;

use poet_filesystem::file_entry::FileEntry;
use poet_filesystem::filesystem::Filesystem as _;
use poet_filesystem::source_directory::SourceDirectory;
use poet_filesystem::source_file::SourceFile;
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

    let mut source_files = storage
        .read_source_files(&SourceDirectory {
            file_extension: "md",
            name: "content",
        })
        .await?;

    source_files.sort_by(|first_source_file, second_source_file| {
        first_source_file
            .stem_path
            .cmp(&second_source_file.stem_path)
    });

    assert_eq!(
        source_files
            .iter()
            .map(|SourceFile { stem_path, .. }| stem_path.clone())
            .collect::<Vec<PathBuf>>(),
        vec![PathBuf::from("docs/intro"), PathBuf::from("guide")]
    );
    assert_eq!(
        source_files
            .iter()
            .map(
                |SourceFile {
                     file_entry: FileEntry { relative_path, .. },
                     ..
                 }| relative_path.clone()
            )
            .collect::<Vec<PathBuf>>(),
        vec![
            PathBuf::from("content/docs/intro.md"),
            PathBuf::from("content/guide.md"),
        ]
    );
    assert_eq!(
        source_files
            .iter()
            .map(
                |SourceFile {
                     file_entry: FileEntry { contents, .. },
                     ..
                 }| contents.as_str()
            )
            .collect::<Vec<&str>>(),
        vec!["intro", "guide"]
    );

    Ok(())
}
