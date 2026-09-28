use poet::poet_error::PoetError;
use tempfile::tempdir;

use crate::fixture_site::FixtureSite;
use crate::poet_tests_error::PoetTestsError;
use crate::run_poet_command::run_poet_command;

#[actix_web::test]
async fn static_pages_reports_invalid_content() -> Result<(), PoetTestsError> {
    let fixture_site = FixtureSite::create()?;
    let output_directory = tempdir()?;
    let output_path = output_directory.path().display().to_string();

    fixture_site.write(
        "content/index.md",
        "+++\ndescription = \"Home\"\nlayout = \"Missing\"\ntitle = \"Home\"\n+++\n",
    )?;

    assert!(matches!(
        run_poet_command(&[
            "make",
            "static-pages",
            "--output-directory",
            &output_path,
            "--public-path",
            "/",
            &fixture_site.path_string()
        ])
        .await?,
        Err(PoetError::BuildProject(_))
    ));

    Ok(())
}
