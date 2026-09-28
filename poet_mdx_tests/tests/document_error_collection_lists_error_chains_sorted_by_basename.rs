use std::path::PathBuf;

use poet_filesystem::filesystem_error::FilesystemError;
use poet_mdx::document_error_collection::DocumentErrorCollection;
use poet_mdx::mdx_error::MdxError;

#[test]
fn document_error_collection_lists_error_chains_sorted_by_basename() {
    let document_error_collection = DocumentErrorCollection::<MdxError>::default();

    document_error_collection.register_error("beta".to_owned(), MdxError::ElementWithoutName);
    document_error_collection.register_error(
        "alpha".to_owned(),
        MdxError::ReadShortcodes(FilesystemError::FileNotFound {
            path: PathBuf::from("shortcodes"),
        }),
    );

    assert_eq!(
        document_error_collection.to_string(),
        "Errors occurred in 2 documents:\nalpha:\n- unable to read shortcode files\n- file 'shortcodes' does not exist\n\nbeta:\n- MDX element has no name\n\n"
    );
}
