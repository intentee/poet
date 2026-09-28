use std::path::PathBuf;

use poet_filesystem::file_entry::FileEntry;
use poet_filesystem::file_entry_stub::FileEntryStub;
use poet_filesystem::source_file::SourceFile;
use poet_filesystem_tests::poet_filesystem_tests_error::PoetFilesystemTestsError;

#[test]
fn source_file_names_stem() -> Result<(), PoetFilesystemTestsError> {
    assert_eq!(
        SourceFile {
            file_entry: FileEntry::from(FileEntryStub {
                contents: String::new(),
                relative_path: PathBuf::from("shortcodes/Layout/Page.rhai"),
            }),
            stem_path: PathBuf::from("Layout/Page"),
        }
        .stem_name()?,
        "Layout/Page"
    );

    Ok(())
}
