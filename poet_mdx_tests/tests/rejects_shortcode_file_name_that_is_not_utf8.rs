use std::ffi::OsString;
use std::fs::create_dir;
use std::fs::write;
use std::os::unix::ffi::OsStringExt as _;

use poet_filesystem::storage::Storage;
use poet_mdx::compile_shortcodes::compile_shortcodes;
use poet_mdx::compile_shortcodes_params::CompileShortcodesParams;
use poet_mdx::mdx_error::MdxError;
use poet_mdx_tests::poet_mdx_tests_error::PoetMdxTestsError;
use poet_mdx_tests::register_fixture_rhai_types::register_fixture_rhai_types;
use tempfile::tempdir;

#[tokio::test]
async fn rejects_shortcode_file_name_that_is_not_utf8() -> Result<(), PoetMdxTestsError> {
    let directory = tempdir()?;

    create_dir(directory.path().join("shortcodes"))?;
    write(
        directory
            .path()
            .join("shortcodes")
            .join(OsString::from_vec(vec![0xff, b'.', b'r', b'h', b'a', b'i'])),
        "fn template(context, props, content) { component { <b /> } }",
    )?;

    let storage = Storage {
        base_directory: directory.path().to_path_buf(),
    };

    assert!(matches!(
        compile_shortcodes(CompileShortcodesParams {
            register_rhai_types: register_fixture_rhai_types,
            source_filesystem: &storage,
        })
        .await,
        Err(MdxError::ResolveShortcodeName(_))
    ));

    Ok(())
}
