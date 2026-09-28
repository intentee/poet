use poet::poet_error::PoetError;
use tempfile::tempdir;

use crate::fixture_site::FixtureSite;
use crate::poet_tests_error::PoetTestsError;
use crate::run_poet_command::run_poet_command;

#[actix_web::test]
async fn static_pages_reports_invalid_esbuild_metafile() -> Result<(), PoetTestsError> {
    let fixture_site = FixtureSite::create()?;
    let output_directory = tempdir()?;
    let output_path = output_directory.path().display().to_string();

    fixture_site.write("esbuild-meta.json", "not json")?;

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
        Err(PoetError::ReadEsbuildMetafile(_))
    ));

    Ok(())
}
