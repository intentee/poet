use std::path::Path;
use std::path::PathBuf;

use poet_filesystem::file_entry::FileEntry;
use poet_filesystem::file_entry_stub::FileEntryStub;
use poet_filesystem::filesystem_error::FilesystemError;
use poet_filesystem::source_directory::SourceDirectory;

#[test]
fn file_entry_rejects_stem_outside_source_directory() {
    let file_entry = FileEntry::from(FileEntryStub {
        contents: String::new(),
        relative_path: PathBuf::from("authors/alice.toml"),
    });

    assert!(matches!(
        file_entry.stem_in(&SourceDirectory {
            file_extension: "md",
            name: "content",
        }),
        Err(FilesystemError::FileOutsideDirectory { directory, .. }) if directory == Path::new("content")
    ));
}
