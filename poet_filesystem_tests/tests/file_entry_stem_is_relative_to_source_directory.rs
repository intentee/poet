use std::path::PathBuf;

use poet_filesystem::file_entry::FileEntry;
use poet_filesystem::file_entry_stub::FileEntryStub;
use poet_filesystem::source_directory::SourceDirectory;
use poet_filesystem_tests::poet_filesystem_tests_error::PoetFilesystemTestsError;

#[test]
fn file_entry_stem_is_relative_to_source_directory() -> Result<(), PoetFilesystemTestsError> {
    let file_entry = FileEntry::from(FileEntryStub {
        contents: String::new(),
        relative_path: PathBuf::from("content/docs/intro.md"),
    });

    assert_eq!(
        file_entry.stem_in(&SourceDirectory {
            file_extension: "md",
            name: "content",
        })?,
        "docs/intro"
    );

    Ok(())
}
