use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt as _;
use std::path::PathBuf;

use poet_filesystem::file_entry::FileEntry;
use poet_filesystem::file_entry_stub::FileEntryStub;
use poet_filesystem::filesystem_error::FilesystemError;
use poet_filesystem::source_directory::SourceDirectory;

#[test]
fn file_entry_rejects_stem_that_is_not_utf8() {
    let file_entry = FileEntry::from(FileEntryStub {
        contents: String::new(),
        relative_path: PathBuf::from("content")
            .join(OsString::from_vec(vec![0xff, b'.', b'm', b'd'])),
    });

    assert!(matches!(
        file_entry.stem_in(&SourceDirectory {
            file_extension: "md",
            name: "content",
        }),
        Err(FilesystemError::NonUtf8Path { .. })
    ));
}
