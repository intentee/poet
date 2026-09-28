use std::fs::write;

use poet_filesystem::storage::Storage;
use poet_mdx::compile_shortcodes::compile_shortcodes;
use poet_mdx::compile_shortcodes_params::CompileShortcodesParams;
use poet_mdx::mdx_error::MdxError;
use poet_mdx_tests::poet_mdx_tests_error::PoetMdxTestsError;
use poet_mdx_tests::register_fixture_rhai_types::register_fixture_rhai_types;
use tempfile::tempdir;

#[tokio::test]
async fn rejects_unreadable_shortcodes_directory() -> Result<(), PoetMdxTestsError> {
    let directory = tempdir()?;
    let regular_file = directory.path().join("regular-file");

    write(&regular_file, "not a directory")?;

    let storage = Storage {
        base_directory: regular_file,
    };

    assert!(matches!(
        compile_shortcodes(CompileShortcodesParams {
            register_rhai_types: register_fixture_rhai_types,
            source_filesystem: &storage,
        })
        .await,
        Err(MdxError::ReadShortcodes(_))
    ));

    Ok(())
}
