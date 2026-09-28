use poet_content::author_collection::AuthorCollection;
use poet_content_tests::evaluate_content_script::evaluate_content_script;
use poet_content_tests::fixture_docs_references::fixture_docs_references;
use poet_content_tests::fixture_site_context::fixture_site_context;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use rhai::Dynamic;

#[test]
fn propagates_nested_hierarchy_callback_failure() -> Result<(), PoetContentTestsError> {
    let site_context =
        fixture_site_context(&fixture_docs_references()?, AuthorCollection::default())?;

    assert!(
        evaluate_content_script::<_, Dynamic>(
            "hierarchy",
            site_context.content_document_collections_ranked["docs"]
                .hierarchy
                .clone(),
            r#"render_hierarchy(hierarchy, |node, level, children| if level == 1 { throw "nested" } else { children })"#,
        )
        .is_err()
    );

    Ok(())
}
