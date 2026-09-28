use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt as _;
use std::path::PathBuf;

use poet_filesystem::file_entry::FileEntry;
use poet_filesystem::file_entry_stub::FileEntryStub;
use poet_filesystem::filesystem_error::FilesystemError;
use poet_filesystem::source_file::SourceFile;

#[test]
fn source_file_rejects_stem_that_is_not_utf8() {
    assert!(matches!(
        SourceFile {
            file_entry: FileEntry::from(FileEntryStub {
                contents: String::new(),
                relative_path: PathBuf::from("shortcodes/invalid.rhai"),
            }),
            stem_path: PathBuf::from(OsString::from_vec(vec![0xff])),
        }
        .stem_name(),
        Err(FilesystemError::NonUtf8Path { .. })
    ));
}
