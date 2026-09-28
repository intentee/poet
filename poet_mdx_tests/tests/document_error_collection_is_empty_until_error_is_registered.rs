use poet_mdx::document_error_collection::DocumentErrorCollection;
use poet_mdx::mdx_error::MdxError;

#[test]
fn document_error_collection_is_empty_until_error_is_registered() {
    let document_error_collection = DocumentErrorCollection::<MdxError>::default();

    assert!(document_error_collection.is_empty());

    document_error_collection.register_error("guide".to_owned(), MdxError::ElementWithoutName);

    assert!(!document_error_collection.is_empty());
}
