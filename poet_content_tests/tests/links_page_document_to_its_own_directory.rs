use std::path::Path;

use poet_content_tests::fixture_reference::fixture_reference;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[test]
fn links_page_document_to_its_own_directory() -> Result<(), PoetContentTestsError> {
    let reference = fixture_reference(
        "docs/guide",
        "description = \"d\"\nlayout = \"L\"\ntitle = \"t\"",
    )?;

    assert_eq!(reference.canonical_link(), "/docs/guide/");
    assert_eq!(
        reference.target_file_relative_path(),
        Path::new("docs/guide/index.html")
    );
    assert_eq!(reference.basename_last_stem(), "guide");

    Ok(())
}
