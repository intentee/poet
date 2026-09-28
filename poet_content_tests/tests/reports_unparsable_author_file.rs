use std::path::Path;

use poet_content::build_authors::build_authors;
use poet_content::content_error::ContentError;
use poet_content_tests::fixture_project::FixtureProject;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use poet_mdx::document_error::DocumentError;

#[tokio::test]
async fn reports_unparsable_author_file() -> Result<(), PoetContentTestsError> {
    let fixture_project = FixtureProject::create()?;

    fixture_project
        .add_file("authors/alice.toml", "unexpected = true")
        .await?;

    let Err(ContentError::InvalidAuthors(author_errors)) =
        build_authors(&fixture_project.storage).await
    else {
        panic!("expected invalid authors");
    };

    assert!(matches!(
        author_errors.into_document_errors().as_slice(),
        [DocumentError {
            error: ContentError::ParseAuthor { relative_path, .. },
            ..
        }] if relative_path == Path::new("authors/alice.toml")
    ));

    Ok(())
}
