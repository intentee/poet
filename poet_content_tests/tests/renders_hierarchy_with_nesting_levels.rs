use poet_content::author_collection::AuthorCollection;
use poet_content_tests::evaluate_content_script::evaluate_content_script;
use poet_content_tests::fixture_docs_references::fixture_docs_references;
use poet_content_tests::fixture_site_context::fixture_site_context;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[test]
fn renders_hierarchy_with_nesting_levels() -> Result<(), PoetContentTestsError> {
    let site_context =
        fixture_site_context(&fixture_docs_references()?, AuthorCollection::default())?;

    assert_eq!(
        evaluate_content_script::<_, String>(
            "hierarchy",
            site_context.content_document_collections_ranked["docs"]
                .hierarchy
                .clone(),
            r#"render_hierarchy(hierarchy, |node, level, children| "[" + node.reference.basename + ":" + node.collection_name + ":" + level + ":" + node.children.len() + children + "]")"#,
        )?,
        "[docs/index:docs:0:1[docs/child:docs:1:0]][docs/second:docs:0:0][docs/hidden:docs:0:0][docs/last:docs:0:0]"
    );

    Ok(())
}
