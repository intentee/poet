use poet_content::author_collection::AuthorCollection;
use poet_content_tests::evaluate_content_script::evaluate_content_script;
use poet_content_tests::fixture_docs_references::fixture_docs_references;
use poet_content_tests::fixture_site_context::fixture_site_context;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[test]
fn hierarchy_has_no_document_before_first_one() -> Result<(), PoetContentTestsError> {
    let site_context =
        fixture_site_context(&fixture_docs_references()?, AuthorCollection::default())?;

    assert_eq!(
        evaluate_content_script::<_, String>(
            "hierarchy",
            site_context.content_document_collections_ranked["docs"]
                .hierarchy
                .clone(),
            r#"type_of(hierarchy.before("docs/index"))"#,
        )?,
        "()"
    );

    Ok(())
}
