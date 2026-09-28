use tempfile::tempdir;

use crate::fixture_site::FixtureSite;
use crate::poet_tests_error::PoetTestsError;
use crate::run_poet_command::run_poet_command;

#[actix_web::test]
async fn static_pages_generates_site() -> Result<(), PoetTestsError> {
    let fixture_site = FixtureSite::create()?;
    let output_directory = tempdir()?;

    assert!(matches!(
        run_poet_command(&[
            "make",
            "static-pages",
            "--output-directory",
            &output_directory.path().display().to_string(),
            "--public-path",
            "/",
            "--sitemap",
            &fixture_site.path_string(),
        ])
        .await?,
        Ok(())
    ));

    assert!(output_directory.path().join("index.html").is_file());
    assert!(
        output_directory
            .path()
            .join("docs/page/index.html")
            .is_file()
    );
    assert!(output_directory.path().join("sitemap.xml").is_file());
    assert!(
        output_directory
            .path()
            .join("static/logo_ABCDEF12.png")
            .is_file()
    );

    Ok(())
}
