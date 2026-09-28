use poet::poet_error::PoetError;

use crate::fixture_site::FixtureSite;
use crate::poet_tests_error::PoetTestsError;
use crate::run_poet_command::run_poet_command;

#[actix_web::test]
async fn serve_reports_invalid_esbuild_metafile() -> Result<(), PoetTestsError> {
    let fixture_site = FixtureSite::create()?;

    fixture_site.write("esbuild-meta.json", "not json")?;

    assert!(matches!(
        run_poet_command(&[
            "serve",
            "--app-name",
            "fixture",
            "--public-path",
            "/",
            &fixture_site.path_string()
        ])
        .await?,
        Err(PoetError::ReadEsbuildMetafile(_))
    ));

    Ok(())
}
