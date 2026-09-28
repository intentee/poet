use std::path::Path;

use poet_assets::asset_manager::AssetManager;
use poet_assets::asset_path_renderer::AssetPathRenderer;
use poet_assets::esbuild_metafile_path::ESBUILD_METAFILE_PATH;
use poet_assets::read_esbuild_metafile_or_default::read_esbuild_metafile_or_default;
use poet_assets_tests::poet_assets_tests_error::PoetAssetsTestsError;
use poet_filesystem::memory::Memory;

#[tokio::test]
async fn reads_and_parses_existing_metafile() -> Result<(), PoetAssetsTestsError> {
    let memory = Memory::default();

    memory.set_file_contents_sync(
        Path::new(ESBUILD_METAFILE_PATH),
        include_str!("../fixtures/entry_with_static_import.json"),
    );

    assert_eq!(
        AssetManager::from_esbuild_metafile(
            read_esbuild_metafile_or_default(&memory).await?,
            AssetPathRenderer {
                base_path: "/".to_owned(),
            },
        )
        .file("logo.png")?,
        "/static/logo_ABCDEF12.png"
    );

    Ok(())
}
