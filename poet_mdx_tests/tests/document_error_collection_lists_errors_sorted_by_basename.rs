use poet_mdx::document_error::DocumentError;
use poet_mdx::document_error_collection::DocumentErrorCollection;
use poet_mdx::mdx_error::MdxError;

#[test]
fn document_error_collection_lists_errors_sorted_by_basename() {
    let document_error_collection = DocumentErrorCollection::<MdxError>::default();

    document_error_collection.register_error("beta".to_owned(), MdxError::ElementWithoutName);
    document_error_collection.register_error("alpha".to_owned(), MdxError::ElementWithoutName);

    assert_eq!(
        document_error_collection
            .into_document_errors()
            .into_iter()
            .map(|DocumentError { basename, .. }| basename)
            .collect::<Vec<String>>(),
        vec!["alpha".to_owned(), "beta".to_owned()]
    );
}
